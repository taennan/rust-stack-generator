use crate::{entity_lowercase}::entity::Model;
use {project_lower}_db_interface::{entity_lowercase}::{entity};

impl From<Model> for {entity} {
    fn from(model: Model) -> Self {
        Self {
{mapped_fields}
        }
    }
}
