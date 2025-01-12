use crate::occupant::entity::ActiveModel;
use db_models::occupant::UpdateOccupantInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateOccupantInputConverter(UpdateOccupantInput);

impl From<UpdateOccupantInput> for UpdateOccupantInputConverter {
    fn from(input: UpdateOccupantInput) -> Self {
        Self(input)
    }
}

impl From<UpdateOccupantInputConverter> for ActiveModel {
    fn from(converter: UpdateOccupantInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			org_id: input.org_id.map_or(NotSet, |v| Set(v)),
			name: input.name.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
