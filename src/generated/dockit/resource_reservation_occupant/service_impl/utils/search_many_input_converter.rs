use crate::resource_reservation_occupant::utils::SearchResourceReservationOccupantInputConverter;
use dockit_db_models::resource_reservation_occupant as db;
use dockit_services_interface::resource_reservation_occupant as service;
use uuid::Uuid;

pub(crate) struct SearchManyResourceReservationOccupantsInputConverter {
    input: service::SearchManyResourceReservationOccupantsInput,
    org_id: Uuid,
}

impl SearchManyResourceReservationOccupantsInputConverter {
    pub fn new(input: service::SearchManyResourceReservationOccupantsInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyResourceReservationOccupantsInputConverter> for db::SearchManyResourceReservationOccupantsInput {
    fn from(converter: SearchManyResourceReservationOccupantsInputConverter) -> Self {
        let conditions_converter = SearchResourceReservationOccupantInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
