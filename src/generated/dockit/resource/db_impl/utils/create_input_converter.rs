use crate::resource::entity::ActiveModel;
use db_models::resource::CreateResourceInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateResourceInputConverter(CreateResourceInput);

impl From<CreateResourceInput> for CreateResourceInputConverter {
    fn from(input: CreateResourceInput) -> Self {
        Self(input)
    }
}

impl From<CreateResourceInputConverter> for ActiveModel {
    fn from(converter: CreateResourceInputConverter) -> Self {
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
