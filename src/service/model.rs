use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Default, Clone)]
pub struct CompleteRequest {
    #[serde(rename = "workerId")]
    pub worker_id: String,
    #[serde(rename = "variables")]
    pub variables: HashMap<String, Variable>,
    #[serde(rename = "localVariables")]
    pub local_variables: HashMap<String, Variable>,
}

#[derive(Serialize, Default, Clone)]
pub struct FetchAndLockRequest {
    #[serde(rename = "workerId")]
    pub worker_id: String,
    #[serde(rename = "maxTasks")]
    pub max_tasks: u32,
    #[serde(rename = "usePriority")]
    pub use_priority: Option<bool>,
    #[serde(rename = "asyncResponseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_response_timeout: Option<u32>,
    #[serde(rename = "topics")]
    pub topics: Vec<Topic>,
}

impl FetchAndLockRequest {
    pub fn new(worker_id: &str) -> Self {
        let mut instance = Self::default();
        instance.worker_id = worker_id.to_string();
        instance
    }

    pub fn topics(&mut self) -> &mut Vec<Topic> {
        &mut self.topics
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct Task {
    #[serde(rename = "activityId")]
    pub activity_id: String,
    #[serde(rename = "activityInstanceId")]
    pub activity_instance_id: String,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "errorDetails")]
    pub error_details: Option<String>,
    #[serde(rename = "executionId")]
    pub execution_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lockExpirationTime")]
    pub lock_expiration_time: String,
    #[serde(rename = "processDefinitionId")]
    pub process_definition_id: String,
    #[serde(rename = "processDefinitionKey")]
    pub process_definition_key: String,
    #[serde(rename = "processDefinitionVersionTag")]
    pub process_definition_version_tag: Option<String>,
    #[serde(rename = "processInstanceId")]
    pub process_instance_id: String,
    #[serde(rename = "retries")]
    pub retries: Option<String>,
    #[serde(rename = "suspended")]
    pub suspended: bool,
    #[serde(rename = "workerId")]
    pub worker_id: String,
    #[serde(rename = "topicName")]
    pub topic_name: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: Option<String>,
    #[serde(rename = "variables")]
    pub variables: HashMap<String, Variable>,
    #[serde(rename = "priority")]
    pub priority: u32,
    #[serde(rename = "businessKey")]
    pub business_key: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Variable {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "value")]
    pub value: Value,
    #[serde(rename = "valueInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_info: Option<ValueInfo>,
}

impl Variable {
    pub fn new(kind: &str, value: Value) -> Self {
        Self {
            kind: kind.to_string(),
            value,
            value_info: None,
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}: {}", self.kind.to_owned(), self.value).as_ref())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ValueInfo {
    #[serde(rename = "objectTypeName")]
    pub object_type_name: Option<String>,
    #[serde(rename = "serializationDataFormat")]
    pub serialization_data_format: Option<String>,
}

#[derive(Serialize, Default, Clone)]
pub struct Topic {
    #[serde(rename = "topicName")]
    pub topic_name: String,
    #[serde(rename = "lockDuration")]
    pub lock_duration: u32,
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<String>>,
    #[serde(rename = "localVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_variables: Option<bool>,
    #[serde(rename = "businessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_key: Option<String>,
    #[serde(rename = "processDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_definition_id: Option<String>,
    #[serde(rename = "processDefinitionIdIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_definition_id_in: Option<Vec<String>>,
    #[serde(rename = "processDefinitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_definition_key: Option<String>,
    #[serde(rename = "processDefinitionKeyIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_definition_key_in: Option<Vec<String>>,
    #[serde(rename = "processDefinitionVersionTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_definition_version_tag: Option<String>,
    #[serde(rename = "withoutTenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_tenant_id: Option<bool>,
    #[serde(rename = "tenantIdIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id_in: Option<Vec<String>>,
    #[serde(rename = "processVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_variables: Option<bool>,
    #[serde(rename = "deserializeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deserialize_values: Option<bool>,
    #[serde(rename = "includeExtensionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_extension_properties: Option<bool>,
}

impl Topic {
    pub fn new(topic_name: &str) -> Self {
        let mut instance = Self::default();
        instance.topic_name = topic_name.to_string();
        instance.lock_duration = 50000;
        instance
    }
}
