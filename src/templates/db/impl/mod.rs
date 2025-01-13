pub mod database;
pub(crate) mod entity;
#[cfg(test)]
mod tests;
pub(crate) mod utils;

pub use database::{{entity}DB, {entity}DBTrait};
