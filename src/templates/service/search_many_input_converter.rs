use crate::{entity_name_lowercase}::utils::Search{entity_name}InputConverter;
use dockit_db_models::{entity_name_lowercase} as db;
use dockit_services_interface::{entity_name_lowercase} as service;
use uuid::Uuid;

pub(crate) struct SearchMany{entity_name}sInputConverter {
    input: service::SearchMany{entity_name}sInput,
    org_id: Uuid,
}

impl SearchMany{entity_name}sInputConverter {
    pub fn new(input: service::SearchMany{entity_name}sInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchMany{entity_name}sInputConverter> for db::SearchMany{entity_name}sInput {
    fn from(converter: SearchMany{entity_name}sInputConverter) -> Self {
        let conditions_converter = Search{entity_name}InputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
