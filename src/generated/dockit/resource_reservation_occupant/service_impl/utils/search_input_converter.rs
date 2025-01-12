use dockit_common_models::search::SearchIdInput;
use dockit_db_models::resource_reservation_occupant as db;
use dockit_services_interface::resource_reservation_occupant as service;
use uuid::Uuid;

pub(crate) struct SearchResourceReservationOccupantInputConverter {
    input: service::SearchResourceReservationOccupantInput,
    org_id: Uuid,
}

impl SearchResourceReservationOccupantInputConverter {
    pub fn new(input: service::SearchResourceReservationOccupantInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchResourceReservationOccupantInputConverter> for db::SearchResourceReservationOccupantInput {
    fn from(converter: SearchResourceReservationOccupantInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
			id: converter.input.id,
			resource_reservation_id: converter.input.resource_reservation_id,
			occupant_id: converter.input.occupant_id,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
