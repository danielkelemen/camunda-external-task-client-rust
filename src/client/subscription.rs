use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{Config, Error, Result, TaskHandler, Topic};

pub struct SubscriptionManager {
    subscriptions: HashMap<String, Arc<RwLock<Topic>>>,
    handlers: HashMap<String, TaskHandler>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    pub fn add_subscription(
        &mut self,
        topic: &Arc<RwLock<Topic>>,
        handler: TaskHandler,
    ) -> Result<()> {
        // FIXME cleanup
        let topic2 = Arc::clone(topic);
        let key: String = topic2.read().unwrap().topic_name.to_owned();

        if self.subscriptions.contains_key(&key) {
            Err(Error::TopicSubscriptionError(
                "topic already registered".to_string(),
            ))
        } else {
            self.subscriptions.insert(key.to_owned(), topic2);
            self.handlers.insert(key.to_owned(), handler);
            Ok(())
        }
    }

    pub fn subscriptions(&self) -> Vec<Topic> {
        self.subscriptions
            .values()
            .map(|t| t.read().unwrap().clone())
            .collect()
    }

    pub fn get_handler(&self, topic_name: &str) -> Option<&TaskHandler> {
        self.handlers.get(topic_name)
    }
}

pub struct SubscriptionBuilder {
    topic: Arc<RwLock<Topic>>,
    handler: Option<TaskHandler>,
    topic_subscription_manager: Arc<RwLock<SubscriptionManager>>,
}

impl SubscriptionBuilder {
    pub fn new(
        config: &Arc<Config>,
        topic_subscription_manager: &Arc<RwLock<SubscriptionManager>>,
        topic_name: &str,
    ) -> Self {
        let instance = Self {
            topic: Arc::new(RwLock::new(Default::default())),
            handler: None,
            topic_subscription_manager: Arc::clone(topic_subscription_manager),
        };

        {
            let mut topic = instance.topic.write().unwrap();
            topic.topic_name = topic_name.to_string();
            topic.lock_duration = config.lock_duration();
        }
        instance
    }

    pub fn handler(&mut self, handler: TaskHandler) -> &mut Self {
        self.handler = Some(handler);
        self
    }

    pub fn lock_duration(&mut self, lock_duration: u32) -> &mut Self {
        self.topic.write().unwrap().lock_duration = lock_duration;
        self
    }

    pub fn variables(&mut self, variables: Option<Vec<String>>) -> &mut Self {
        self.topic.write().unwrap().variables = variables;
        self
    }

    pub fn local_variables(&mut self, local_variables: Option<bool>) -> &mut Self {
        self.topic.write().unwrap().local_variables = local_variables;
        self
    }

    pub fn business_key(&mut self, business_key: Option<String>) -> &mut Self {
        self.topic.write().unwrap().business_key = business_key;
        self
    }

    pub fn process_definition_id(&mut self, process_definition_id: Option<String>) -> &mut Self {
        self.topic.write().unwrap().process_definition_id = process_definition_id;
        self
    }

    pub fn process_definition_id_in(
        &mut self,
        process_definition_id_in: Option<Vec<String>>,
    ) -> &mut Self {
        self.topic.write().unwrap().process_definition_id_in = process_definition_id_in;
        self
    }

    pub fn process_definition_key(&mut self, process_definition_key: Option<String>) -> &mut Self {
        self.topic.write().unwrap().process_definition_key = process_definition_key;
        self
    }

    pub fn process_definition_key_in(
        &mut self,
        process_definition_key_in: Option<Vec<String>>,
    ) -> &mut Self {
        self.topic.write().unwrap().process_definition_key_in = process_definition_key_in;
        self
    }

    pub fn process_definition_version_tag(
        &mut self,
        process_definition_version_tag: Option<String>,
    ) -> &mut Self {
        self.topic.write().unwrap().process_definition_version_tag = process_definition_version_tag;
        self
    }

    pub fn without_tenant_id(&mut self, without_tenant_id: Option<bool>) -> &mut Self {
        self.topic.write().unwrap().without_tenant_id = without_tenant_id;
        self
    }

    pub fn tenant_id_in(&mut self, tenant_id_in: Option<Vec<String>>) -> &mut Self {
        self.topic.write().unwrap().tenant_id_in = tenant_id_in;
        self
    }

    pub fn process_variables(&mut self, process_variables: Option<bool>) -> &mut Self {
        self.topic.write().unwrap().process_variables = process_variables;
        self
    }

    pub fn deserialize_values(&mut self, deserialize_values: Option<bool>) -> &mut Self {
        self.topic.write().unwrap().deserialize_values = deserialize_values;
        self
    }

    pub fn include_extension_properties(
        &mut self,
        include_extension_properties: Option<bool>,
    ) -> &mut Self {
        self.topic.write().unwrap().include_extension_properties = include_extension_properties;
        self
    }

    pub fn open(&mut self) -> Result<()> {
        if self.handler.is_none() {
            Err(Error::TopicSubscriptionError(
                "handler cannot be none".to_string(),
            ))
        } else {
            let mut tsm = self.topic_subscription_manager.write().unwrap();
            tsm.add_subscription(&self.topic, self.handler.take().unwrap())?;
            Ok(())
        }
    }
}
