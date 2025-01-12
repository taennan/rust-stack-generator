use core_services_interface::resource as service;
use db_interface::resource as db;

pub(crate) struct UpdateResourceInputConverter(service::UpdateResourceInput);

impl From<service::UpdateResourceInput> for UpdateResourceInputConverter {
    fn from(input: service::UpdateResourceInput) -> Self {
        Self(input)
    }
}

impl From<UpdateResourceInputConverter> for db::UpdateResourceInput {
    fn from(converter: UpdateResourceInputConverter) -> Self {
        let input = converter.0;
        Self {
			id: converter.input.id,
			name: converter.input.name,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
