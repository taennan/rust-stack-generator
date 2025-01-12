use dockit_common_models::search::SearchIdInput;
use dockit_db_models::reservee as db;
use dockit_services_interface::reservee as service;
use uuid::Uuid;

pub(crate) struct SearchReserveeInputConverter {
    input: service::SearchReserveeInput,
    org_id: Uuid,
}

impl SearchReserveeInputConverter {
    pub fn new(input: service::SearchReserveeInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchReserveeInputConverter> for db::SearchReserveeInput {
    fn from(converter: SearchReserveeInputConverter) -> Self {
        Self {
            org_id: Some(SearchIdInput::equals(converter.org_id)),
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
