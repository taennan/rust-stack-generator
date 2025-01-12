use crate::org::entity::ActiveModel;
use db_models::org::CreateOrgInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateOrgInputConverter(CreateOrgInput);

impl From<CreateOrgInput> for CreateOrgInputConverter {
    fn from(input: CreateOrgInput) -> Self {
        Self(input)
    }
}

impl From<CreateOrgInputConverter> for ActiveModel {
    fn from(converter: CreateOrgInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			name: Set(input.name),
			description: Set(input.description),
            ..Default::default()
        }
    }
}
