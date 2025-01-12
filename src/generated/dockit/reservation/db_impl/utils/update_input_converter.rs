use crate::reservation::entity::ActiveModel;
use db_models::reservation::UpdateReservationInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateReservationInputConverter(UpdateReservationInput);

impl From<UpdateReservationInput> for UpdateReservationInputConverter {
    fn from(input: UpdateReservationInput) -> Self {
        Self(input)
    }
}

impl From<UpdateReservationInputConverter> for ActiveModel {
    fn from(converter: UpdateReservationInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			org_id: input.org_id.map_or(NotSet, |v| Set(v)),
			reservee_id: input.reservee_id.map_or(NotSet, |v| Set(v)),
			status: input.status.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
