pub mod database;
pub mod models;

pub use models::*;
pub use database::OccupantDB;
#[cfg(test)]
pub use database::MockOccupantDB;
