use std::collections::HashMap;
use std::sync::Arc;

use log::{error, trace};
use reqwest::blocking::{Client, ClientBuilder, Response};
use reqwest::Result as ReqwestResult;
use serde::Serialize;

use crate::{CompleteRequest, Config, Error, FetchAndLockRequest, Result, Task, Variable};

pub struct EngineService {
    config: Arc<Config>,
    client: Client,
}

impl EngineService {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        // TODO offer attributes to configure client or to pass it via parameter
        let client = ClientBuilder::new()
            .build()
            .map_err(|e| Error::ClientConfigurationError(e))?;
        Ok(Self { config, client })
    }

    fn post<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> ReqwestResult<Response> {
        let mut url = self.config.base_url().to_owned();
        url.push_str(path);
        let request = self.client.post(url).json(body).build().unwrap();
        trace!("{request:?}");
        trace!("{:?}", request.body());
        self.client.execute(request)
    }

    pub fn complete(
        &self,
        task: &Task,
        variables: Option<HashMap<String, Variable>>,
        local_variables: Option<HashMap<String, Variable>>,
    ) -> Result<()> {
        let path = format!("/external-task/{}/complete", task.id);
        let mut request = CompleteRequest {
            worker_id: self.config.worker_id().into(),
            variables: Default::default(),
            local_variables: Default::default(),
        };
        if let Some(variables) = variables {
            request.variables = variables;
        }
        if let Some(local_variables) = local_variables {
            request.local_variables = local_variables;
        }
        let res = self.post(path.as_ref(), &request);

        match res {
            Ok(response) => {
                trace!("{response:?}");
                let response_text = response.text().unwrap();
                trace!("{response_text:?}");
                Ok(())
            }
            Err(e) => {
                error!("{e:?}");
                Err(Error::BackendRequestError("request failed".to_string()))
            }
        }
    }

    pub fn fetch_and_lock(&self, request: FetchAndLockRequest) -> Result<Vec<Task>> {
        let res = self.post("/external-task/fetchAndLock", &request);

        match res {
            Ok(response) => {
                trace!("{response:?}");
                let response_text = response.text().unwrap();
                trace!("{response_text:?}");
                serde_json::from_str(response_text.as_ref()).map_err(|e| {
                    error!("{e:?}");
                    Error::BackendRequestError("cannot deserialize json".to_string())
                })
            }
            Err(e) => {
                error!("{e:?}");
                Err(Error::BackendRequestError("request failed".to_string()))
            }
        }
    }
}
