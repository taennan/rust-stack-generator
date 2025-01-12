use crate::org_user::entity::ActiveModel;
use db_models::org_user::UpdateOrgUserInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateOrgUserInputConverter(UpdateOrgUserInput);

impl From<UpdateOrgUserInput> for UpdateOrgUserInputConverter {
    fn from(input: UpdateOrgUserInput) -> Self {
        Self(input)
    }
}

impl From<UpdateOrgUserInputConverter> for ActiveModel {
    fn from(converter: UpdateOrgUserInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			org_id: input.org_id.map_or(NotSet, |v| Set(v)),
			claw_auth_id: input.claw_auth_id.map_or(NotSet, |v| Set(v)),
			email: input.email.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
