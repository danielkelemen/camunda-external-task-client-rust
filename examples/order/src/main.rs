use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, Local, Timelike, Utc};
use log::{info, LevelFilter};
use serde_json::Value;

use camunda_external_task_client_rust::*;

fn invoice_creator_handler(task: &Task, engine_service: &EngineService) {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let date_time: DateTime<Utc> = SystemTime::now().into();
    variables.insert(
        "date".into(),
        Variable::new("string", Value::String(date_time.to_rfc3339())),
    );
    // TODO read invoice.txt

    let minute = Local::now().minute();
    let _ = if minute % 2 == 0 {
        engine_service.complete(task, Some(variables), None)
    } else {
        engine_service.complete(task, None, Some(variables))
    }
    .unwrap();
}

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    info!("starting client");

    let config = Config::new("http://localhost:8080/engine-rest")
        .with_interval(1000)
        .with_max_tasks(100);
    let mut client = Client::new(config)?;

    // subscribe to topic
    client
        .subscribe("invoiceCreator")
        .handler(invoice_creator_handler)
        .open()?;

    // wait for client thread
    client.join()?;

    Ok(())
}
