use crate::resource::utils::SearchResourceInputConverter;
use dockit_db_models::resource as db;
use dockit_services_interface::resource as service;
use uuid::Uuid;

pub(crate) struct SearchManyResourcesInputConverter {
    input: service::SearchManyResourcesInput,
    org_id: Uuid,
}

impl SearchManyResourcesInputConverter {
    pub fn new(input: service::SearchManyResourcesInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyResourcesInputConverter> for db::SearchManyResourcesInput {
    fn from(converter: SearchManyResourcesInputConverter) -> Self {
        let conditions_converter = SearchResourceInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
