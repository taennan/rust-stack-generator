use {project_lower}_services_interface::{entity_lower} as service;
use {project_lower}_db_interface::{entity_lower} as db;

pub(crate) struct {entity}Converter(db::{entity});

impl From<db::{entity}> for {entity}Converter {
    fn from(model: db::{entity}) -> Self {
        Self(model)
    }
}

impl From<{entity}Converter> for service::{entity} {
    fn from(converter: {entity}Converter) -> Self {
        let model = converter.0;
        Self {
{mapped_fields}
        }
    }
}
