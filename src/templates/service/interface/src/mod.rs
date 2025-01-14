pub mod models;
pub mod service;

pub use models::*;
pub use service::{entity}Service;
pub type {entity}ServiceTrait = {entity}Service;
#[cfg(test)]
pub use service::Mock{entity}Service;