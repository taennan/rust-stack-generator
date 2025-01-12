use dockit_common_models::search::SearchIdInput;
use dockit_db_models::reservee_user as db;
use dockit_services_interface::reservee_user as service;
use uuid::Uuid;

pub(crate) struct SearchReserveeUserInputConverter {
    input: service::SearchReserveeUserInput,
    org_id: Uuid,
}

impl SearchReserveeUserInputConverter {
    pub fn new(input: service::SearchReserveeUserInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchReserveeUserInputConverter> for db::SearchReserveeUserInput {
    fn from(converter: SearchReserveeUserInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
			id: converter.input.id,
			claw_auth_id: converter.input.claw_auth_id,
			email: converter.input.email,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
