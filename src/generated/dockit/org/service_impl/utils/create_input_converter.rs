use dockit_db_models::org as db;
use dockit_services_interface::org as service;
use uuid::Uuid;

pub(crate) struct CreateOrgInputConverter {
    input: service::CreateOrgInput,
    org_id: Uuid,
}

impl CreateOrgInputConverter {
    pub fn new(input: service::CreateOrgInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateOrgInputConverter> for db::CreateOrgInput {
    fn from(converter: CreateOrgInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			name: converter.input.name,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
