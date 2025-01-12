use dockit_db_models::org_user as db;
use dockit_services_interface::org_user as service;
use uuid::Uuid;

pub(crate) struct CreateOrgUserInputConverter {
    input: service::CreateOrgUserInput,
    org_id: Uuid,
}

impl CreateOrgUserInputConverter {
    pub fn new(input: service::CreateOrgUserInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateOrgUserInputConverter> for db::CreateOrgUserInput {
    fn from(converter: CreateOrgUserInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			claw_auth_id: converter.input.claw_auth_id,
			email: converter.input.email,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
