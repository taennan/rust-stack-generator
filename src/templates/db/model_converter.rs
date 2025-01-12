use crate::{entity_name_lowercase}::entity::Model;
use db_interface::{entity_name_lowercase}::{entity_name};

impl From<Model> for {entity_name} {
    fn from(model: Model) -> Self {
        Self {
{mapped_fields}
        }
    }
}
