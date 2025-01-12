use dockit_common_models::search::SearchIdInput;
use dockit_db_models::org_user as db;
use dockit_services_interface::org_user as service;
use uuid::Uuid;

pub(crate) struct SearchOrgUserInputConverter {
    input: service::SearchOrgUserInput,
    org_id: Uuid,
}

impl SearchOrgUserInputConverter {
    pub fn new(input: service::SearchOrgUserInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchOrgUserInputConverter> for db::SearchOrgUserInput {
    fn from(converter: SearchOrgUserInputConverter) -> Self {
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
