use crate::reservation::utils::SearchReservationInputConverter;
use dockit_db_models::reservation as db;
use dockit_services_interface::reservation as service;
use uuid::Uuid;

pub(crate) struct SearchManyReservationsInputConverter {
    input: service::SearchManyReservationsInput,
    org_id: Uuid,
}

impl SearchManyReservationsInputConverter {
    pub fn new(input: service::SearchManyReservationsInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyReservationsInputConverter> for db::SearchManyReservationsInput {
    fn from(converter: SearchManyReservationsInputConverter) -> Self {
        let conditions_converter = SearchReservationInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
