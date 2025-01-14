use crate::{entity_lowercase}::utils::Search{entity}InputConverter;
use {project_lower}_db_models::{entity_lowercase} as db;
use {project_lower}_services_interface::{entity_lowercase} as service;
use uuid::Uuid;

pub(crate) struct SearchMany{entity}sInputConverter {
    input: service::SearchMany{entity}sInput,
    org_id: Uuid,
}

impl SearchMany{entity}sInputConverter {
    pub fn new(input: service::SearchMany{entity}sInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchMany{entity}sInputConverter> for db::SearchMany{entity}sInput {
    fn from(converter: SearchMany{entity}sInputConverter) -> Self {
        let conditions_converter = Search{entity}InputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
