pub mod database;
pub mod models;

pub use models::*;
pub use database::{{entity}DB, {entity}DBTrait};
#[cfg(test)]
pub use database::Mock{entity}DB;
