use crate::occupant::utils::SearchOccupantInputConverter;
use dockit_db_models::occupant as db;
use dockit_services_interface::occupant as service;
use uuid::Uuid;

pub(crate) struct SearchManyOccupantsInputConverter {
    input: service::SearchManyOccupantsInput,
    org_id: Uuid,
}

impl SearchManyOccupantsInputConverter {
    pub fn new(input: service::SearchManyOccupantsInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyOccupantsInputConverter> for db::SearchManyOccupantsInput {
    fn from(converter: SearchManyOccupantsInputConverter) -> Self {
        let conditions_converter = SearchOccupantInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
