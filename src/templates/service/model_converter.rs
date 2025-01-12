use core_services_interface::{entity_name_lowercase} as service;
use db_interface::{entity_name_lowercase} as db;

pub(crate) struct {entity_name}Converter(db::{entity_name});

impl From<db::{entity_name}> for {entity_name}Converter {
    fn from(model: db::{entity_name}) -> Self {
        Self(model)
    }
}

impl From<{entity_name}Converter> for service::{entity_name} {
    fn from(converter: {entity_name}Converter) -> Self {
        let model = converter.0;
        Self {
{mapped_fields}
        }
    }
}
