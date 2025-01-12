use dockit_common_models::search::SearchIdInput;
use dockit_db_models::reservation as db;
use dockit_services_interface::reservation as service;
use uuid::Uuid;

pub(crate) struct SearchReservationInputConverter {
    input: service::SearchReservationInput,
    org_id: Uuid,
}

impl SearchReservationInputConverter {
    pub fn new(input: service::SearchReservationInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchReservationInputConverter> for db::SearchReservationInput {
    fn from(converter: SearchReservationInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
			id: converter.input.id,
			reservee_id: converter.input.reservee_id,
			status: converter.input.status,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
