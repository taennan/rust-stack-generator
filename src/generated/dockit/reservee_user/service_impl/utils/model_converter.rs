use core_services_interface::reservee_user as service;
use db_interface::reservee_user as db;

pub(crate) struct ReserveeUserConverter(db::ReserveeUser);

impl From<db::ReserveeUser> for ReserveeUserConverter {
    fn from(model: db::ReserveeUser) -> Self {
        Self(model)
    }
}

impl From<ReserveeUserConverter> for service::ReserveeUser {
    fn from(converter: ReserveeUserConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			claw_auth_id: model.claw_auth_id,
			email: model.email,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
