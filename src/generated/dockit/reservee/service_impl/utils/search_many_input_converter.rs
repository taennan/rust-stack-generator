use crate::reservee::utils::SearchReserveeInputConverter;
use dockit_db_models::reservee as db;
use dockit_services_interface::reservee as service;
use uuid::Uuid;

pub(crate) struct SearchManyReserveesInputConverter {
    input: service::SearchManyReserveesInput,
    org_id: Uuid,
}

impl SearchManyReserveesInputConverter {
    pub fn new(input: service::SearchManyReserveesInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyReserveesInputConverter> for db::SearchManyReserveesInput {
    fn from(converter: SearchManyReserveesInputConverter) -> Self {
        let conditions_converter = SearchReserveeInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
