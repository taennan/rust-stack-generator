use crate::admin_api_key::entity::Model;
use db_interface::admin_api_key::AdminApiKey;

impl From<Model> for AdminApiKey {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			hash: model.hash,
			created: model.created,
        }
    }
}
