use core_services_interface::reservation as service;
use db_interface::reservation as db;

pub(crate) struct UpdateReservationInputConverter(service::UpdateReservationInput);

impl From<service::UpdateReservationInput> for UpdateReservationInputConverter {
    fn from(input: service::UpdateReservationInput) -> Self {
        Self(input)
    }
}

impl From<UpdateReservationInputConverter> for db::UpdateReservationInput {
    fn from(converter: UpdateReservationInputConverter) -> Self {
        let input = converter.0;
        Self {
			id: converter.input.id,
			reservee_id: converter.input.reservee_id,
			status: converter.input.status,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
