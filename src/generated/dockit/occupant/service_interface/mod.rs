pub mod models;
pub mod service;

pub use models::*;
pub use service::OccupantService;
#[cfg(test)]
pub use service::MockOccupantService;