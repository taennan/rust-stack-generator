use crate::resource_reservation_occupant::entity::Model;
use db_interface::resource_reservation_occupant::ResourceReservationOccupant;

impl From<Model> for ResourceReservationOccupant {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			org_id: model.org_id,
			resource_reservation_id: model.resource_reservation_id,
			occupant_id: model.occupant_id,
			created: model.created,
			updated: model.updated,
        }
    }
}
