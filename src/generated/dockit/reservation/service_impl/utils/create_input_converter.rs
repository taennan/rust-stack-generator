use dockit_db_models::reservation as db;
use dockit_services_interface::reservation as service;
use uuid::Uuid;

pub(crate) struct CreateReservationInputConverter {
    input: service::CreateReservationInput,
    org_id: Uuid,
}

impl CreateReservationInputConverter {
    pub fn new(input: service::CreateReservationInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateReservationInputConverter> for db::CreateReservationInput {
    fn from(converter: CreateReservationInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			reservee_id: converter.input.reservee_id,
			status: converter.input.status,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
