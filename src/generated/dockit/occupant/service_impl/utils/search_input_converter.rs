use dockit_common_models::search::SearchIdInput;
use dockit_db_models::occupant as db;
use dockit_services_interface::occupant as service;
use uuid::Uuid;

pub(crate) struct SearchOccupantInputConverter {
    input: service::SearchOccupantInput,
    org_id: Uuid,
}

impl SearchOccupantInputConverter {
    pub fn new(input: service::SearchOccupantInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchOccupantInputConverter> for db::SearchOccupantInput {
    fn from(converter: SearchOccupantInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
			id: converter.input.id,
			name: converter.input.name,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
