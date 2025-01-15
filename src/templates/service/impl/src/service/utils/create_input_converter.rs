use {project_lower}_db_models::{entity_lower} as db;
use {project_lower}_services_interface::{entity_lower} as service;
use uuid::Uuid;

pub(crate) struct Create{entity}InputConverter {
    input: service::Create{entity}Input,
    org_id: Uuid,
}

impl Create{entity}InputConverter {
    pub fn new(input: service::Create{entity}Input, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<Create{entity}InputConverter> for db::Create{entity}Input {
    fn from(converter: Create{entity}InputConverter) -> Self {
        Self {
            org_id: converter.org_id,
{mapped_fields}
        }
    }
}
