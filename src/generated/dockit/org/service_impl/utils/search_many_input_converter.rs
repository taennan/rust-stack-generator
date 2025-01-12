use crate::org::utils::SearchOrgInputConverter;
use dockit_db_models::org as db;
use dockit_services_interface::org as service;
use uuid::Uuid;

pub(crate) struct SearchManyOrgsInputConverter {
    input: service::SearchManyOrgsInput,
    org_id: Uuid,
}

impl SearchManyOrgsInputConverter {
    pub fn new(input: service::SearchManyOrgsInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyOrgsInputConverter> for db::SearchManyOrgsInput {
    fn from(converter: SearchManyOrgsInputConverter) -> Self {
        let conditions_converter = SearchOrgInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
