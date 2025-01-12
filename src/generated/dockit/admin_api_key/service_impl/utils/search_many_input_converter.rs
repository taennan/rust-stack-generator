use crate::admin_api_key::utils::SearchAdminApiKeyInputConverter;
use dockit_db_models::admin_api_key as db;
use dockit_services_interface::admin_api_key as service;
use uuid::Uuid;

pub(crate) struct SearchManyAdminApiKeysInputConverter {
    input: service::SearchManyAdminApiKeysInput,
    org_id: Uuid,
}

impl SearchManyAdminApiKeysInputConverter {
    pub fn new(input: service::SearchManyAdminApiKeysInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyAdminApiKeysInputConverter> for db::SearchManyAdminApiKeysInput {
    fn from(converter: SearchManyAdminApiKeysInputConverter) -> Self {
        let conditions_converter = SearchAdminApiKeyInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
