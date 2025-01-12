use crate::reservee::entity::ActiveModel;
use db_models::reservee::CreateReserveeInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateReserveeInputConverter(CreateReserveeInput);

impl From<CreateReserveeInput> for CreateReserveeInputConverter {
    fn from(input: CreateReserveeInput) -> Self {
        Self(input)
    }
}

impl From<CreateReserveeInputConverter> for ActiveModel {
    fn from(converter: CreateReserveeInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			org_id: Set(input.org_id),
			reservee_user_id: Set(input.reservee_user_id),
			first_name: Set(input.first_name),
			middle_names: Set(input.middle_names),
			last_name: Set(input.last_name),
			email: Set(input.email),
			phone: Set(input.phone),
			description: Set(input.description),
            ..Default::default()
        }
    }
}
