use crate::admin_api_key::entity::ActiveModel;
use db_models::admin_api_key::CreateAdminApiKeyInput;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct CreateAdminApiKeyInputConverter(CreateAdminApiKeyInput);

impl From<CreateAdminApiKeyInput> for CreateAdminApiKeyInputConverter {
    fn from(input: CreateAdminApiKeyInput) -> Self {
        Self(input)
    }
}

impl From<CreateAdminApiKeyInputConverter> for ActiveModel {
    fn from(converter: CreateAdminApiKeyInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
			hash: Set(input.hash),
            ..Default::default()
        }
    }
}
