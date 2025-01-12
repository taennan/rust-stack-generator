use crate::reservation::entity::Model;
use db_interface::reservation::Reservation;

impl From<Model> for Reservation {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			org_id: model.org_id,
			reservee_id: model.reservee_id,
			status: model.status,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
