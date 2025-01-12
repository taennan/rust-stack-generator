use crate::org_user::entity::ActiveModel;
use db_models::org_user::CreateOrgUserInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateOrgUserInputConverter(CreateOrgUserInput);

impl From<CreateOrgUserInput> for CreateOrgUserInputConverter {
    fn from(input: CreateOrgUserInput) -> Self {
        Self(input)
    }
}

impl From<CreateOrgUserInputConverter> for ActiveModel {
    fn from(converter: CreateOrgUserInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			org_id: Set(input.org_id),
			claw_auth_id: Set(input.claw_auth_id),
			email: Set(input.email),
			description: Set(input.description),
            ..Default::default()
        }
    }
}
