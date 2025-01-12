use dockit_db_models::reservee as db;
use dockit_services_interface::reservee as service;
use uuid::Uuid;

pub(crate) struct CreateReserveeInputConverter {
    input: service::CreateReserveeInput,
    org_id: Uuid,
}

impl CreateReserveeInputConverter {
    pub fn new(input: service::CreateReserveeInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<CreateReserveeInputConverter> for db::CreateReserveeInput {
    fn from(converter: CreateReserveeInputConverter) -> Self {
        Self {
            org_id: converter.org_id,
			id: converter.input.id,
			reservee_user_id: converter.input.reservee_user_id,
			first_name: converter.input.first_name,
			middle_names: converter.input.middle_names,
			last_name: converter.input.last_name,
			email: converter.input.email,
			phone: converter.input.phone,
			description: converter.input.description,
			created: converter.input.created,
			updated: converter.input.updated,
        }
    }
}
