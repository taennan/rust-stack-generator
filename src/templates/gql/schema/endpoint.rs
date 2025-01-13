use crate::utils::endpoint_impl::gql_endpoints;
use {project_lower}_services_interface::{entity_lowercase}::{
    Create{entity}Input, {entity}, {entity}Service, SearchMany{entity}sInput, Search{entity}Input,
    Update{entity}Input,
};
use uuid::Uuid;

gql_endpoints! {
    {entity}Queries,

    get_{entity_lowercase}_by_id(id: Uuid) -> {entity} {
        {entity_lowercase}_service
        get_by_id_or_throw
    }

    get_one_{entity_lowercase}(input: Search{entity}Input) -> Option<{entity}> {
        {entity_lowercase}_service
        get_one
    }

    get_many_{entity_lowercase}s(input: SearchMany{entity}sInput) -> Vec<{entity}> {
        {entity_lowercase}_service
        get_many
    }
}

gql_endpoints! {
    {entity}Mutations,

    create_{entity_lowercase}(input: Create{entity}Input) -> {entity} {
        {entity_lowercase}_service
        create
    }

    update_{entity_lowercase}(input: Update{entity}Input) -> {entity} {
        {entity_lowercase}_service
        update
    }
}
