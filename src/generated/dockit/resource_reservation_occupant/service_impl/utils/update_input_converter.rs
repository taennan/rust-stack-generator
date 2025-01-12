use core_services_interface::resource_reservation_occupant as service;
use db_interface::resource_reservation_occupant as db;

pub(crate) struct UpdateResourceReservationOccupantInputConverter(service::UpdateResourceReservationOccupantInput);

impl From<service::UpdateResourceReservationOccupantInput> for UpdateResourceReservationOccupantInputConverter {
    fn from(input: service::UpdateResourceReservationOccupantInput) -> Self {
        Self(input)
    }
}

impl From<UpdateResourceReservationOccupantInputConverter> for db::UpdateResourceReservationOccupantInput {
    fn from(converter: UpdateResourceReservationOccupantInputConverter) -> Self {
        let input = converter.0;
        Self {
			id: converter.input.id,
			resource_reservation_id: converter.input.resource_reservation_id,
			occupant_id: converter.input.occupant_id,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
