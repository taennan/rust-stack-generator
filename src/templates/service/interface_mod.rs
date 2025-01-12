pub mod models;
pub mod service;

pub use models::*;
pub use service::{entity_name}Service;
#[cfg(test)]
pub use service::Mock{entity_name}Service;