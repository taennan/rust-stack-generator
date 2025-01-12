use dockit_db_models::{entity_name_lowercase} as db;
use dockit_services_interface::{entity_name_lowercase} as service;
use uuid::Uuid;

pub(crate) struct Create{entity_name}InputConverter {
    input: service::Create{entity_name}Input,
    org_id: Uuid,
}

impl Create{entity_name}InputConverter {
    pub fn new(input: service::Create{entity_name}Input, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<Create{entity_name}InputConverter> for db::Create{entity_name}Input {
    fn from(converter: Create{entity_name}InputConverter) -> Self {
        Self {
            org_id: converter.org_id,
{mapped_fields}
        }
    }
}
