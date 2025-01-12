use crate::reservee_user::entity::ActiveModel;
use db_models::reservee_user::UpdateReserveeUserInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateReserveeUserInputConverter(UpdateReserveeUserInput);

impl From<UpdateReserveeUserInput> for UpdateReserveeUserInputConverter {
    fn from(input: UpdateReserveeUserInput) -> Self {
        Self(input)
    }
}

impl From<UpdateReserveeUserInputConverter> for ActiveModel {
    fn from(converter: UpdateReserveeUserInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			claw_auth_id: input.claw_auth_id.map_or(NotSet, |v| Set(v)),
			email: input.email.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
