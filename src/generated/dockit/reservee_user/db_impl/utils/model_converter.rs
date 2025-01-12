use crate::reservee_user::entity::Model;
use db_interface::reservee_user::ReserveeUser;

impl From<Model> for ReserveeUser {
    fn from(model: Model) -> Self {
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
