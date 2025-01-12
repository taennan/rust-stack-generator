use crate::org::entity::Model;
use db_interface::org::Org;

impl From<Model> for Org {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			name: model.name,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
