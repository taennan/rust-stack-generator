use crate::reservee_user::entity::ActiveModel;
use db_models::reservee_user::CreateReserveeUserInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateReserveeUserInputConverter(CreateReserveeUserInput);

impl From<CreateReserveeUserInput> for CreateReserveeUserInputConverter {
    fn from(input: CreateReserveeUserInput) -> Self {
        Self(input)
    }
}

impl From<CreateReserveeUserInputConverter> for ActiveModel {
    fn from(converter: CreateReserveeUserInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			claw_auth_id: Set(input.claw_auth_id),
			email: Set(input.email),
			description: Set(input.description),
            ..Default::default()
        }
    }
}
