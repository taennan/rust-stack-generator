use core_services_interface::admin_api_key as service;
use db_interface::admin_api_key as db;

pub(crate) struct UpdateAdminApiKeyInputConverter(service::UpdateAdminApiKeyInput);

impl From<service::UpdateAdminApiKeyInput> for UpdateAdminApiKeyInputConverter {
    fn from(input: service::UpdateAdminApiKeyInput) -> Self {
        Self(input)
    }
}

impl From<UpdateAdminApiKeyInputConverter> for db::UpdateAdminApiKeyInput {
    fn from(converter: UpdateAdminApiKeyInputConverter) -> Self {
        let input = converter.0;
        Self {
			id: converter.input.id,
			hash: converter.input.hash,
			created: converter.input.created,
        }
    }
}
