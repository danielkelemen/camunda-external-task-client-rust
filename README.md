# camunda-external-task-client

Camunda external task client implemented using the rust language.

## State of the project

**The project is not complete**, but it is already in a usable state.
You can use the client to subscribe to topics and complete tasks.

# Installing

The project is not available on [crates.io](https://crates.io/) yet, so the only way to use it right now is via github url:

```toml
[dependencies]
camunda-external-task-client = { git = "https://github.com/danielkelemen/camunda-external-task-client-rust" }
```

# Usage

```rust
use camunda_external_task_client_rust::*;

fn main() -> Result<()> {
    // activate logger
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // configure client
    let config = Config::new("http://localhost:8080/engine-rest")
        .with_interval(1000)
        .with_max_tasks(100);
    let mut client = Client::new(config)?;

    // subscribe to topic
    client
        .subscribe("invoiceCreator")
        .handler(|task: &Task, engine_service: &EngineService| {
            info!("received task {}", task.id);
        })
        .open()?;

    // wait for client thread
    client.join()?;

    Ok(())
}
```

## Examples

Take a look into the [examples](https://github.com/danielkelemen/camunda-external-task-client-rust/examples) folder in the source code.

# Contributing

Feel free to create issues and PRs as there are many missing things (docs, tests, APIs etc.).

# License

Apache License Version 2.0.