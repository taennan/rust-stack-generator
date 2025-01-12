use core_services_interface::admin_api_key as service;
use db_interface::admin_api_key as db;

pub(crate) struct AdminApiKeyConverter(db::AdminApiKey);

impl From<db::AdminApiKey> for AdminApiKeyConverter {
    fn from(model: db::AdminApiKey) -> Self {
        Self(model)
    }
}

impl From<AdminApiKeyConverter> for service::AdminApiKey {
    fn from(converter: AdminApiKeyConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			hash: model.hash,
			created: model.created,
        }
    }
}
