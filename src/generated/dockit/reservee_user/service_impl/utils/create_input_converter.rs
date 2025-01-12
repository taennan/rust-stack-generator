use dockit_db_models::reservee_user as db;
use dockit_services_interface::reservee_user as service;
use uuid::Uuid;

pub(crate) struct CreateReserveeUserInputConverter {
    input: service::CreateReserveeUserInput,
    org_id: Uuid,
}

impl CreateReserveeUserInputConverter {
    pub fn new(input: service::CreateReserveeUserInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateReserveeUserInputConverter> for db::CreateReserveeUserInput {
    fn from(converter: CreateReserveeUserInputConverter) -> Self {
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
