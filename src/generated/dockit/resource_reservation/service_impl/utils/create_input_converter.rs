use dockit_db_models::resource_reservation as db;
use dockit_services_interface::resource_reservation as service;
use uuid::Uuid;

pub(crate) struct CreateResourceReservationInputConverter {
    input: service::CreateResourceReservationInput,
    org_id: Uuid,
}

impl CreateResourceReservationInputConverter {
    pub fn new(input: service::CreateResourceReservationInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateResourceReservationInputConverter> for db::CreateResourceReservationInput {
    fn from(converter: CreateResourceReservationInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			resource_id: converter.input.resource_id,
			reservation_id: converter.input.reservation_id,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
