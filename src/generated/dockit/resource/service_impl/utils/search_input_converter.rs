use dockit_common_models::search::SearchIdInput;
use dockit_db_models::resource as db;
use dockit_services_interface::resource as service;
use uuid::Uuid;

pub(crate) struct SearchResourceInputConverter {
    input: service::SearchResourceInput,
    org_id: Uuid,
}

impl SearchResourceInputConverter {
    pub fn new(input: service::SearchResourceInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchResourceInputConverter> for db::SearchResourceInput {
    fn from(converter: SearchResourceInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
			id: converter.input.id,
			name: converter.input.name,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
