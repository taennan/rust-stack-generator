use crate::reservee::entity::Model;
use db_interface::reservee::Reservee;

impl From<Model> for Reservee {
    fn from(model: Model) -> Self {
        Self {
			id: model.id,
			org_id: model.org_id,
			reservee_user_id: model.reservee_user_id,
			first_name: model.first_name,
			middle_names: model.middle_names,
			last_name: model.last_name,
			email: model.email,
			phone: model.phone,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
