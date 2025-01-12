use core_services_interface::resource_reservation as service;
use db_interface::resource_reservation as db;

pub(crate) struct ResourceReservationConverter(db::ResourceReservation);

impl From<db::ResourceReservation> for ResourceReservationConverter {
    fn from(model: db::ResourceReservation) -> Self {
        Self(model)
    }
}

impl From<ResourceReservationConverter> for service::ResourceReservation {
    fn from(converter: ResourceReservationConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			resource_id: model.resource_id,
			reservation_id: model.reservation_id,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
