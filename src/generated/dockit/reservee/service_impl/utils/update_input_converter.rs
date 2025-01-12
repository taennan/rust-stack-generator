use core_services_interface::reservee as service;
use db_interface::reservee as db;

pub(crate) struct UpdateReserveeInputConverter(service::UpdateReserveeInput);

impl From<service::UpdateReserveeInput> for UpdateReserveeInputConverter {
    fn from(input: service::UpdateReserveeInput) -> Self {
        Self(input)
    }
}

impl From<UpdateReserveeInputConverter> for db::UpdateReserveeInput {
    fn from(converter: UpdateReserveeInputConverter) -> Self {
        let input = converter.0;
        Self {
			id: converter.input.id,
			reservee_user_id: converter.input.reservee_user_id,
			first_name: converter.input.first_name,
			middle_names: converter.input.middle_names,
			last_name: converter.input.last_name,
			email: converter.input.email,
			phone: converter.input.phone,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
