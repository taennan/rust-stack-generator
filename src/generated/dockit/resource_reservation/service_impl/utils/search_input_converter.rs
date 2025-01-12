use dockit_common_models::search::SearchIdInput;
use dockit_db_models::resource_reservation as db;
use dockit_services_interface::resource_reservation as service;
use uuid::Uuid;

pub(crate) struct SearchResourceReservationInputConverter {
    input: service::SearchResourceReservationInput,
    org_id: Uuid,
}

impl SearchResourceReservationInputConverter {
    pub fn new(input: service::SearchResourceReservationInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchResourceReservationInputConverter> for db::SearchResourceReservationInput {
    fn from(converter: SearchResourceReservationInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
			id: converter.input.id,
			resource_id: converter.input.resource_id,
			reservation_id: converter.input.reservation_id,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
