use {project_lower}_common_models::search::SearchIdInput;
use {project_lower}_db_models::{entity_lowercase} as db;
use {project_lower}_services_interface::{entity_lowercase} as service;
use uuid::Uuid;

pub(crate) struct Search{entity}InputConverter {
    input: service::Search{entity}Input,
    org_id: Uuid,
}

impl Search{entity}InputConverter {
    pub fn new(input: service::Search{entity}Input, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<Search{entity}InputConverter> for db::Search{entity}Input {
    fn from(converter: Search{entity}InputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::Equals(converter.org_id)),
{mapped_fields}
        }
    }
}
