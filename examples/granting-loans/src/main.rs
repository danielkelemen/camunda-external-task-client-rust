use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use log::{error, info, LevelFilter};
use serde_json::Value;

use camunda_external_task_client_rust::*;

fn request_rejecter_handler(task: &Task, engine_service: &EngineService) {
    if let Some(bar) = task.variables.get("bar") {
        info!("bar: {bar}");
    }
    if let Some(credit_scores) = task.variables.get("creditScores") {
        info!("creditScores: {credit_scores}");
    }

    let res_complete = engine_service.complete(task, None, None);
    match res_complete {
        Ok(_) => info!("Rejection processed"),
        Err(e) => error!("Failed to process rejection, {e:?}"),
    };
}

fn credit_score_checker_handler(task: &Task, engine_service: &EngineService) {
    let default_score = task
        .variables
        .get("defaultScore")
        .unwrap()
        .value
        .as_u64()
        .unwrap();
    info!("defaultScore: {default_score}");

    let credit_scores = vec![default_score, 9, 1, 4, 10];
    let mut process_variables: HashMap<String, Variable> = HashMap::new();
    process_variables.insert(
        "creditScores".into(),
        Variable::new("json", Value::String(format!("{credit_scores:?}"))),
    );
    let date_time: DateTime<Utc> = SystemTime::now().into();
    process_variables.insert(
        "bar".into(),
        Variable::new("string", Value::String(date_time.to_rfc3339())),
    );

    let res_complete = engine_service.complete(task, Some(process_variables), None);
    match res_complete {
        Ok(_) => info!("I completed my task successfully!!"),
        Err(e) => error!("Failed completing my task, {e:?}"),
    };
}

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    info!("starting client");

    let config = Config::new("http://localhost:8080/engine-rest") //
        .with_interval(1000)
        .with_max_tasks(100);
    let mut client = Client::new(config)?;

    // subscribe to topic
    client
        .subscribe("creditScoreChecker")
        .handler(credit_score_checker_handler)
        .open()?;

    client
        .subscribe("requestRejecter")
        .handler(request_rejecter_handler)
        .open()?;

    // wait for client thread
    client.join()?;

    Ok(())
}
