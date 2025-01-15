use {project_lower}_services_interface::{entity_lower} as service;
use {project_lower}_db_interface::{entity_lower} as db;

pub(crate) struct Update{entity}InputConverter(service::Update{entity}Input);

impl From<service::Update{entity}Input> for Update{entity}InputConverter {
    fn from(input: service::Update{entity}Input) -> Self {
        Self(input)
    }
}

impl From<Update{entity}InputConverter> for db::Update{entity}Input {
    fn from(converter: Update{entity}InputConverter) -> Self {
        let input = converter.0;
        Self {
{mapped_fields}
        }
    }
}
