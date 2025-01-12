use core_services_interface::org as service;
use db_interface::org as db;

pub(crate) struct UpdateOrgInputConverter(service::UpdateOrgInput);

impl From<service::UpdateOrgInput> for UpdateOrgInputConverter {
    fn from(input: service::UpdateOrgInput) -> Self {
        Self(input)
    }
}

impl From<UpdateOrgInputConverter> for db::UpdateOrgInput {
    fn from(converter: UpdateOrgInputConverter) -> Self {
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
