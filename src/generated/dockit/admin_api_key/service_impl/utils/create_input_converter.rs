use dockit_db_models::admin_api_key as db;
use dockit_services_interface::admin_api_key as service;
use uuid::Uuid;

pub(crate) struct CreateAdminApiKeyInputConverter {
    input: service::CreateAdminApiKeyInput,
    org_id: Uuid,
}

impl CreateAdminApiKeyInputConverter {
    pub fn new(input: service::CreateAdminApiKeyInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateAdminApiKeyInputConverter> for db::CreateAdminApiKeyInput {
    fn from(converter: CreateAdminApiKeyInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			hash: converter.input.hash,
			created: converter.input.created,
        }
    }
}
