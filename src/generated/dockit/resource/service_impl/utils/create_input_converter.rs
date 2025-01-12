use dockit_db_models::resource as db;
use dockit_services_interface::resource as service;
use uuid::Uuid;

pub(crate) struct CreateResourceInputConverter {
    input: service::CreateResourceInput,
    org_id: Uuid,
}

impl CreateResourceInputConverter {
    pub fn new(input: service::CreateResourceInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateResourceInputConverter> for db::CreateResourceInput {
    fn from(converter: CreateResourceInputConverter) -> Self {
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
