use crate::resource_reservation_occupant::entity::ActiveModel;
use db_models::resource_reservation_occupant::CreateResourceReservationOccupantInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateResourceReservationOccupantInputConverter(CreateResourceReservationOccupantInput);

impl From<CreateResourceReservationOccupantInput> for CreateResourceReservationOccupantInputConverter {
    fn from(input: CreateResourceReservationOccupantInput) -> Self {
        Self(input)
    }
}

impl From<CreateResourceReservationOccupantInputConverter> for ActiveModel {
    fn from(converter: CreateResourceReservationOccupantInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			org_id: Set(input.org_id),
			resource_reservation_id: Set(input.resource_reservation_id),
			occupant_id: Set(input.occupant_id),
            ..Default::default()
        }
    }
}
