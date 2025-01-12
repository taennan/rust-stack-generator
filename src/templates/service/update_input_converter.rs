use core_services_interface::{entity_name_lowercase} as service;
use db_interface::{entity_name_lowercase} as db;

pub(crate) struct Update{entity_name}InputConverter(service::Update{entity_name}Input);

impl From<service::Update{entity_name}Input> for Update{entity_name}InputConverter {
    fn from(input: service::Update{entity_name}Input) -> Self {
        Self(input)
    }
}

impl From<Update{entity_name}InputConverter> for db::Update{entity_name}Input {
    fn from(converter: Update{entity_name}InputConverter) -> Self {
        let input = converter.0;
        Self {
{mapped_fields}
        }
    }
}
