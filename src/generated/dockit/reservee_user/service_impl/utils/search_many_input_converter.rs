use crate::reservee_user::utils::SearchReserveeUserInputConverter;
use dockit_db_models::reservee_user as db;
use dockit_services_interface::reservee_user as service;
use uuid::Uuid;

pub(crate) struct SearchManyReserveeUsersInputConverter {
    input: service::SearchManyReserveeUsersInput,
    org_id: Uuid,
}

impl SearchManyReserveeUsersInputConverter {
    pub fn new(input: service::SearchManyReserveeUsersInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyReserveeUsersInputConverter> for db::SearchManyReserveeUsersInput {
    fn from(converter: SearchManyReserveeUsersInputConverter) -> Self {
        let conditions_converter = SearchReserveeUserInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
