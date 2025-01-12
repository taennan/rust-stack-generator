use core_services_interface::reservee_user as service;
use db_interface::reservee_user as db;

pub(crate) struct UpdateReserveeUserInputConverter(service::UpdateReserveeUserInput);

impl From<service::UpdateReserveeUserInput> for UpdateReserveeUserInputConverter {
    fn from(input: service::UpdateReserveeUserInput) -> Self {
        Self(input)
    }
}

impl From<UpdateReserveeUserInputConverter> for db::UpdateReserveeUserInput {
    fn from(converter: UpdateReserveeUserInputConverter) -> Self {
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
