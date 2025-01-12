use core_services_interface::org_user as service;
use db_interface::org_user as db;

pub(crate) struct UpdateOrgUserInputConverter(service::UpdateOrgUserInput);

impl From<service::UpdateOrgUserInput> for UpdateOrgUserInputConverter {
    fn from(input: service::UpdateOrgUserInput) -> Self {
        Self(input)
    }
}

impl From<UpdateOrgUserInputConverter> for db::UpdateOrgUserInput {
    fn from(converter: UpdateOrgUserInputConverter) -> Self {
        let input = converter.0;
        Self {
			id: converter.input.id,
			claw_auth_id: converter.input.claw_auth_id,
			email: converter.input.email,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
