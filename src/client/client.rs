//! TODO: add docs
//!
//! ```
//! use camunda_external_task_client_rust::*;
//!
//! let config = Config::new("http://...");
//! client
//!     .subscribe("topic")
//!     .handler(|task: &Task, engine_service: &EngineService| {
//!         // your code
//!     })
//!     .open()?;
//! ```
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use log::{error, trace};
use rayon::prelude::*;

use crate::{
    Config, EngineService, Error, FetchAndLockRequest, Result, SubscriptionBuilder,
    SubscriptionManager, Task,
};

/// Camunda external task client.
pub struct Client {
    config: Arc<Config>,
    engine_service: Arc<EngineService>,
    subscription_manager: Arc<RwLock<SubscriptionManager>>,
    task_polling: TaskPolling,
    poll_join_handle: Option<JoinHandle<()>>,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        let config = Arc::new(config);
        let engine_service = EngineService::new(Arc::clone(&config))?;
        let engine_service = Arc::new(engine_service);
        let topic_subscription_manager = Arc::new(RwLock::new(SubscriptionManager::new()));

        let mut instance = Self {
            task_polling: TaskPolling::new(config.interval()),
            subscription_manager: topic_subscription_manager,
            poll_join_handle: None,
            config,
            engine_service,
        };

        if instance.config.auto_poll() {
            instance.poll_join_handle = Some(instance.start()?);
        }

        Ok(instance)
    }

    pub fn engine_service(&self) -> &Arc<EngineService> {
        &self.engine_service
    }

    pub fn topic_subscription_manager(&self) -> &Arc<RwLock<SubscriptionManager>> {
        &self.subscription_manager
    }

    pub fn start(&mut self) -> Result<JoinHandle<()>> {
        let subscription_manager = Arc::clone(&self.subscription_manager);
        let config = Arc::clone(&self.config);
        let engine_service = Arc::clone(&self.engine_service);

        self.task_polling.start(move || {
            // fetch
            let subscriptions = subscription_manager.read().unwrap().subscriptions();
            let mut request = FetchAndLockRequest::new(config.worker_id());
            request.topics = subscriptions;
            request.use_priority = Some(config.use_priority());
            request.max_tasks = config.max_tasks();
            let tasks = engine_service.fetch_and_lock(request).unwrap();

            // group tasks by activityId
            let mut tasks_by_activity: HashMap<String, Vec<Task>> = HashMap::new();
            tasks.into_iter().for_each(|task| {
                tasks_by_activity
                    .entry(task.activity_id.to_owned())
                    .or_insert_with(Vec::new)
                    .push(task)
            });

            // execute tasks
            tasks_by_activity.par_iter().for_each(|(_, tasks)| {
                tasks.iter().for_each(|task| {
                    let subscription_manager = subscription_manager.read().unwrap();
                    let handler_option = subscription_manager.get_handler(&task.topic_name);
                    if let Some(handler) = handler_option {
                        handler(task, &engine_service)
                    }
                });
            });
        })
    }

    pub fn stop(&mut self) {
        self.task_polling.stop();
    }

    pub fn is_active(&self) -> bool {
        self.task_polling.is_active()
    }

    pub fn subscribe(&self, topic_name: &str) -> SubscriptionBuilder {
        SubscriptionBuilder::new(&self.config, &self.subscription_manager, topic_name)
    }

    pub fn join(&mut self) -> Result<()> {
        if self.poll_join_handle.is_none() {
            return Err(Error::ClientError(
                "client not active, cannot join".to_string(),
            ));
        }

        self.poll_join_handle.take().unwrap().join().map_err(|e| {
            error!("{e:?}");
            Error::ClientError("could not join client thread".to_string())
        })
    }
}

struct TaskPolling {
    is_active: Arc<RwLock<bool>>,
    interval: u64,
}

impl TaskPolling {
    pub fn new(interval: u64) -> Self {
        Self {
            is_active: Arc::new(RwLock::new(false)),
            interval,
        }
    }

    pub fn start<F>(&mut self, callback: F) -> Result<JoinHandle<()>>
    where
        F: Fn() -> () + 'static + Send,
    {
        if *self.is_active.read().unwrap() {
            return Err(Error::ClientError("client already started".to_string()));
        }
        trace!("start");
        *self.is_active.write().unwrap() = true;
        self.poll(callback)
    }

    pub fn stop(&mut self) {
        trace!("stop");
        *self.is_active.write().unwrap() = false;
    }

    pub fn is_active(&self) -> bool {
        *self.is_active.write().unwrap()
    }

    fn poll<F>(&self, callback: F) -> Result<JoinHandle<()>>
    where
        F: Fn() -> () + 'static + Send,
    {
        let is_active = Arc::clone(&self.is_active);
        let millis = self.interval;
        let handler = thread::Builder::new()
            .name("client-internal-polling-runtime".into())
            .spawn(move || {
                while *is_active.read().unwrap() {
                    trace!("polling...");
                    callback();
                    thread::sleep(Duration::from_millis(millis));
                }
            })
            .map_err(|e| {
                error!("{e:?}");
                Error::ClientError("failed to spawn polling thread".into())
            })?;
        trace!("polling stopped");
        Ok(handler)
    }
}
