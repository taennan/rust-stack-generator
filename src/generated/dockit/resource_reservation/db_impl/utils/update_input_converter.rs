use crate::resource_reservation::entity::ActiveModel;
use db_models::resource_reservation::UpdateResourceReservationInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateResourceReservationInputConverter(UpdateResourceReservationInput);

impl From<UpdateResourceReservationInput> for UpdateResourceReservationInputConverter {
    fn from(input: UpdateResourceReservationInput) -> Self {
        Self(input)
    }
}

impl From<UpdateResourceReservationInputConverter> for ActiveModel {
    fn from(converter: UpdateResourceReservationInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			org_id: input.org_id.map_or(NotSet, |v| Set(v)),
			resource_id: input.resource_id.map_or(NotSet, |v| Set(v)),
			reservation_id: input.reservation_id.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
