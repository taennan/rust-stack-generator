use crate::occupant::entity::ActiveModel;
use db_models::occupant::CreateOccupantInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateOccupantInputConverter(CreateOccupantInput);

impl From<CreateOccupantInput> for CreateOccupantInputConverter {
    fn from(input: CreateOccupantInput) -> Self {
        Self(input)
    }
}

impl From<CreateOccupantInputConverter> for ActiveModel {
    fn from(converter: CreateOccupantInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			org_id: Set(input.org_id),
			name: Set(input.name),
			description: Set(input.description),
            ..Default::default()
        }
    }
}
