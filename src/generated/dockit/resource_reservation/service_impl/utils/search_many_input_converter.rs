use crate::resource_reservation::utils::SearchResourceReservationInputConverter;
use dockit_db_models::resource_reservation as db;
use dockit_services_interface::resource_reservation as service;
use uuid::Uuid;

pub(crate) struct SearchManyResourceReservationsInputConverter {
    input: service::SearchManyResourceReservationsInput,
    org_id: Uuid,
}

impl SearchManyResourceReservationsInputConverter {
    pub fn new(input: service::SearchManyResourceReservationsInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyResourceReservationsInputConverter> for db::SearchManyResourceReservationsInput {
    fn from(converter: SearchManyResourceReservationsInputConverter) -> Self {
        let conditions_converter = SearchResourceReservationInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
