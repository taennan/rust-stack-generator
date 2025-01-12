use dockit_common_models::search::SearchIdInput;
use dockit_db_models::org as db;
use dockit_services_interface::org as service;
use uuid::Uuid;

pub(crate) struct SearchOrgInputConverter {
    input: service::SearchOrgInput,
    org_id: Uuid,
}

impl SearchOrgInputConverter {
    pub fn new(input: service::SearchOrgInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchOrgInputConverter> for db::SearchOrgInput {
    fn from(converter: SearchOrgInputConverter) -> Self {
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
