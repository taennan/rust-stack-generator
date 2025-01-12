use dockit_common_models::search::SearchIdInput;
use dockit_db_models::admin_api_key as db;
use dockit_services_interface::admin_api_key as service;
use uuid::Uuid;

pub(crate) struct SearchAdminApiKeyInputConverter {
    input: service::SearchAdminApiKeyInput,
    org_id: Uuid,
}

impl SearchAdminApiKeyInputConverter {
    pub fn new(input: service::SearchAdminApiKeyInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchAdminApiKeyInputConverter> for db::SearchAdminApiKeyInput {
    fn from(converter: SearchAdminApiKeyInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
			id: converter.input.id,
			hash: converter.input.hash,
			created: converter.input.created,
        }
    }
}
