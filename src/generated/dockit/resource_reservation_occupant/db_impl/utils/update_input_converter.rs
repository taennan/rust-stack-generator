use crate::resource_reservation_occupant::entity::ActiveModel;
use db_models::resource_reservation_occupant::UpdateResourceReservationOccupantInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateResourceReservationOccupantInputConverter(UpdateResourceReservationOccupantInput);

impl From<UpdateResourceReservationOccupantInput> for UpdateResourceReservationOccupantInputConverter {
    fn from(input: UpdateResourceReservationOccupantInput) -> Self {
        Self(input)
    }
}

impl From<UpdateResourceReservationOccupantInputConverter> for ActiveModel {
    fn from(converter: UpdateResourceReservationOccupantInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			org_id: input.org_id.map_or(NotSet, |v| Set(v)),
			resource_reservation_id: input.resource_reservation_id.map_or(NotSet, |v| Set(v)),
			occupant_id: input.occupant_id.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
