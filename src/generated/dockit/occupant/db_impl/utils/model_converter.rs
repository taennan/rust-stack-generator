use crate::occupant::entity::Model;
use db_interface::occupant::Occupant;

impl From<Model> for Occupant {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			org_id: model.org_id,
			name: model.name,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
