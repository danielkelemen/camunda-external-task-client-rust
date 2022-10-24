pub use client::*;
pub use config::*;
pub use subscription::*;

use crate::{EngineService, Task};

mod client;
mod config;
mod subscription;

pub type TaskHandler = fn(&Task, &EngineService) -> ();
