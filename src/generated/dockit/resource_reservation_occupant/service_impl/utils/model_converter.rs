use core_services_interface::resource_reservation_occupant as service;
use db_interface::resource_reservation_occupant as db;

pub(crate) struct ResourceReservationOccupantConverter(db::ResourceReservationOccupant);

impl From<db::ResourceReservationOccupant> for ResourceReservationOccupantConverter {
    fn from(model: db::ResourceReservationOccupant) -> Self {
        Self(model)
    }
}

impl From<ResourceReservationOccupantConverter> for service::ResourceReservationOccupant {
    fn from(converter: ResourceReservationOccupantConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			resource_reservation_id: model.resource_reservation_id,
			occupant_id: model.occupant_id,
			created: model.created,
			updated: model.updated,
        }
    }
}
