pub mod database;
pub mod models;

pub use models::*;
pub use database::{entity_name}DB;
#[cfg(test)]
pub use database::Mock{entity_name}DB;
