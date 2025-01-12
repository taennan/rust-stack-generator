use crate::resource_reservation::entity::ActiveModel;
use db_models::resource_reservation::CreateResourceReservationInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateResourceReservationInputConverter(CreateResourceReservationInput);

impl From<CreateResourceReservationInput> for CreateResourceReservationInputConverter {
    fn from(input: CreateResourceReservationInput) -> Self {
        Self(input)
    }
}

impl From<CreateResourceReservationInputConverter> for ActiveModel {
    fn from(converter: CreateResourceReservationInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			org_id: Set(input.org_id),
			resource_id: Set(input.resource_id),
			reservation_id: Set(input.reservation_id),
			description: Set(input.description),
            ..Default::default()
        }
    }
}
