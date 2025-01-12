use core_services_interface::occupant as service;
use db_interface::occupant as db;

pub(crate) struct UpdateOccupantInputConverter(service::UpdateOccupantInput);

impl From<service::UpdateOccupantInput> for UpdateOccupantInputConverter {
    fn from(input: service::UpdateOccupantInput) -> Self {
        Self(input)
    }
}

impl From<UpdateOccupantInputConverter> for db::UpdateOccupantInput {
    fn from(converter: UpdateOccupantInputConverter) -> Self {
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
