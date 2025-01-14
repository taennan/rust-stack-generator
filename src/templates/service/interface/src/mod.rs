pub mod models;
pub mod service;

pub use models::*;
pub use service::{{entity}Service, {entity}ServiceTrait};
#[cfg(test)]
pub use service::Mock{entity}Service;
