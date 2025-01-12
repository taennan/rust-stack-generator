use dockit_common_models::search::SearchIdInput;
use dockit_db_models::{entity_name_lowercase} as db;
use dockit_services_interface::{entity_name_lowercase} as service;
use uuid::Uuid;

pub(crate) struct Search{entity_name}InputConverter {
    input: service::Search{entity_name}Input,
    org_id: Uuid,
}

impl Search{entity_name}InputConverter {
    pub fn new(input: service::Search{entity_name}Input, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<Search{entity_name}InputConverter> for db::Search{entity_name}Input {
    fn from(converter: Search{entity_name}InputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
{mapped_fields}
        }
    }
}
