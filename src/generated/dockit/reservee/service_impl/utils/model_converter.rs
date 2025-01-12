use core_services_interface::reservee as service;
use db_interface::reservee as db;

pub(crate) struct ReserveeConverter(db::Reservee);

impl From<db::Reservee> for ReserveeConverter {
    fn from(model: db::Reservee) -> Self {
        Self(model)
    }
}

impl From<ReserveeConverter> for service::Reservee {
    fn from(converter: ReserveeConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			reservee_user_id: model.reservee_user_id,
			first_name: model.first_name,
			middle_names: model.middle_names,
			last_name: model.last_name,
			email: model.email,
			phone: model.phone,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
