use crate::org_user::entity::Model;
use db_interface::org_user::OrgUser;

impl From<Model> for OrgUser {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			org_id: model.org_id,
			claw_auth_id: model.claw_auth_id,
			email: model.email,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
