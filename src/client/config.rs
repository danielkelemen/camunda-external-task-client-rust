pub struct Config {
    base_url: String,
    worker_id: String,
    max_tasks: u32,
    use_priority: bool,
    interval: u64,
    lock_duration: u32,
    auto_poll: bool,
    interceptors: Vec<Interceptor>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "".to_string(),
            worker_id: "some-random-id".to_string(),
            max_tasks: 10,
            use_priority: true,
            interval: 1000,
            lock_duration: 50000,
            auto_poll: true,
            interceptors: vec![],
        }
    }
}

impl Config {
    pub fn new(base_url: &str) -> Self {
        let mut config = Self::default();
        config.base_url = base_url.to_string();
        config
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn worker_id(&self) -> &str {
        &self.worker_id
    }
    pub fn max_tasks(&self) -> u32 {
        self.max_tasks
    }
    pub fn use_priority(&self) -> bool {
        self.use_priority
    }
    pub fn interval(&self) -> u64 {
        self.interval
    }
    pub fn lock_duration(&self) -> u32 {
        self.lock_duration
    }
    pub fn auto_poll(&self) -> bool {
        self.auto_poll
    }
    pub fn interceptors(&self) -> &Vec<Interceptor> {
        &self.interceptors
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
    pub fn with_worker_id(mut self, worker_id: &str) -> Self {
        self.worker_id = worker_id.to_string();
        self
    }
    pub fn with_max_tasks(mut self, max_tasks: u32) -> Self {
        self.max_tasks = max_tasks;
        self
    }
    pub fn with_use_priority(mut self, use_priority: bool) -> Self {
        self.use_priority = use_priority;
        self
    }
    pub fn with_interval(mut self, interval: u64) -> Self {
        self.interval = interval;
        self
    }
    pub fn with_lock_duration(mut self, lock_duration: u32) -> Self {
        self.lock_duration = lock_duration;
        self
    }
    pub fn with_auto_poll(mut self, auto_poll: bool) -> Self {
        self.auto_poll = auto_poll;
        self
    }
    pub fn with_interceptors(mut self, interceptors: Vec<Interceptor>) -> Self {
        self.interceptors = interceptors;
        self
    }
}

// TODO not implemented yet
pub struct Interceptor {}

// FIXME impl or remove ConfigBuilder -> https://docs.rs/derive_builder/latest/derive_builder/
//pub struct ConfigBuilder {
//    config: Config,
//}
//
// impl ConfigBuilder {
//     pub fn new() -> Self {
//         Self {
//             config: Default::default(),
//         }
//     }
//
//     pub fn with_base_url(&mut self, base_url: &str) -> &mut Self {
//         self.config.base_url = base_url.to_string();
//         self
//     }
//     pub fn with_worker_id(&mut self, worker_id: String) -> &mut Self {
//         self.config.worker_id = worker_id;
//         self
//     }
//     pub fn with_max_tasks(&mut self, max_tasks: u32) -> &mut Self {
//         self.config.max_tasks = max_tasks;
//         self
//     }
//     pub fn with_use_priority(&mut self, use_priority: bool) -> &mut Self {
//         self.config.use_priority = use_priority;
//         self
//     }
//     pub fn with_interval(&mut self, interval: u64) -> &mut Self {
//         self.config.interval = interval;
//         self
//     }
//     pub fn with_lock_duration(&mut self, lock_duration: u32) -> &mut Self {
//         self.config.lock_duration = lock_duration;
//         self
//     }
//     pub fn with_auto_poll(&mut self, auto_poll: bool) -> &mut Self {
//         self.config.auto_poll = auto_poll;
//         self
//     }
//     pub fn with_interceptors(&mut self, interceptors: Vec<Interceptor>) -> &mut Self {
//         self.config.interceptors = interceptors;
//         self
//     }
//
//     pub fn build(self) -> Config {
//         self.config
//     }
// }
