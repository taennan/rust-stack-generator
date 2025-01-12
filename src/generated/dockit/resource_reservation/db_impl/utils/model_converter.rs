use crate::resource_reservation::entity::Model;
use db_interface::resource_reservation::ResourceReservation;

impl From<Model> for ResourceReservation {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			org_id: model.org_id,
			resource_id: model.resource_id,
			reservation_id: model.reservation_id,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
