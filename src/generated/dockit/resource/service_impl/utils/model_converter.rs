use core_services_interface::resource as service;
use db_interface::resource as db;

pub(crate) struct ResourceConverter(db::Resource);

impl From<db::Resource> for ResourceConverter {
    fn from(model: db::Resource) -> Self {
        Self(model)
    }
}

impl From<ResourceConverter> for service::Resource {
    fn from(converter: ResourceConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			name: model.name,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
