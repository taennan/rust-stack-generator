use core_services_interface::resource_reservation as service;
use db_interface::resource_reservation as db;

pub(crate) struct UpdateResourceReservationInputConverter(service::UpdateResourceReservationInput);

impl From<service::UpdateResourceReservationInput> for UpdateResourceReservationInputConverter {
    fn from(input: service::UpdateResourceReservationInput) -> Self {
        Self(input)
    }
}

impl From<UpdateResourceReservationInputConverter> for db::UpdateResourceReservationInput {
    fn from(converter: UpdateResourceReservationInputConverter) -> Self {
        let input = converter.0;
        Self {
			id: converter.input.id,
			resource_id: converter.input.resource_id,
			reservation_id: converter.input.reservation_id,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
