use dockit_db_models::occupant as db;
use dockit_services_interface::occupant as service;
use uuid::Uuid;

pub(crate) struct CreateOccupantInputConverter {
    input: service::CreateOccupantInput,
    org_id: Uuid,
}

impl CreateOccupantInputConverter {
    pub fn new(input: service::CreateOccupantInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateOccupantInputConverter> for db::CreateOccupantInput {
    fn from(converter: CreateOccupantInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			name: converter.input.name,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
