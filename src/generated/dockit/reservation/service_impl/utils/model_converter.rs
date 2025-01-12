use core_services_interface::reservation as service;
use db_interface::reservation as db;

pub(crate) struct ReservationConverter(db::Reservation);

impl From<db::Reservation> for ReservationConverter {
    fn from(model: db::Reservation) -> Self {
        Self(model)
    }
}

impl From<ReservationConverter> for service::Reservation {
    fn from(converter: ReservationConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			reservee_id: model.reservee_id,
			status: model.status,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
