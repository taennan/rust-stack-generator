use crate::admin_api_key::entity::ActiveModel;
use db_models::admin_api_key::UpdateAdminApiKeyInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateAdminApiKeyInputConverter(UpdateAdminApiKeyInput);

impl From<UpdateAdminApiKeyInput> for UpdateAdminApiKeyInputConverter {
    fn from(input: UpdateAdminApiKeyInput) -> Self {
        Self(input)
    }
}

impl From<UpdateAdminApiKeyInputConverter> for ActiveModel {
    fn from(converter: UpdateAdminApiKeyInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			hash: input.hash.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
