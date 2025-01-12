use crate::org_user::utils::SearchOrgUserInputConverter;
use dockit_db_models::org_user as db;
use dockit_services_interface::org_user as service;
use uuid::Uuid;

pub(crate) struct SearchManyOrgUsersInputConverter {
    input: service::SearchManyOrgUsersInput,
    org_id: Uuid,
}

impl SearchManyOrgUsersInputConverter {
    pub fn new(input: service::SearchManyOrgUsersInput, org_id: Uuid) -> Self {
        Self { input, org_id }
    }
}

impl From<SearchManyOrgUsersInputConverter> for db::SearchManyOrgUsersInput {
    fn from(converter: SearchManyOrgUsersInputConverter) -> Self {
        let conditions_converter = SearchOrgUserInputConverter::new(
            converter.input.conditions.unwrap_or_default(),
            converter.org_id,
        );
        Self {
            conditions: Some(conditions_converter.into()),
            pagination: converter.input.pagination,
        }
    }
}
