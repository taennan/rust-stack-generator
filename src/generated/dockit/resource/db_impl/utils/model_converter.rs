use crate::resource::entity::Model;
use db_interface::resource::Resource;

impl From<Model> for Resource {
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
