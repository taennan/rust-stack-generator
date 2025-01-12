use dockit_db_models::resource_reservation_occupant as db;
use dockit_services_interface::resource_reservation_occupant as service;
use uuid::Uuid;

pub(crate) struct CreateResourceReservationOccupantInputConverter {
    input: service::CreateResourceReservationOccupantInput,
    org_id: Uuid,
}

impl CreateResourceReservationOccupantInputConverter {
    pub fn new(input: service::CreateResourceReservationOccupantInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateResourceReservationOccupantInputConverter> for db::CreateResourceReservationOccupantInput {
    fn from(converter: CreateResourceReservationOccupantInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			resource_reservation_id: converter.input.resource_reservation_id,
			occupant_id: converter.input.occupant_id,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
