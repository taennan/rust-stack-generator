use crate::reservation::entity::ActiveModel;
use db_models::reservation::CreateReservationInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateReservationInputConverter(CreateReservationInput);

impl From<CreateReservationInput> for CreateReservationInputConverter {
    fn from(input: CreateReservationInput) -> Self {
        Self(input)
    }
}

impl From<CreateReservationInputConverter> for ActiveModel {
    fn from(converter: CreateReservationInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			org_id: Set(input.org_id),
			reservee_id: Set(input.reservee_id),
			status: Set(input.status),
			description: Set(input.description),
            ..Default::default()
        }
    }
}
