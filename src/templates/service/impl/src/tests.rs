use crate::{
    {entity_lowercase}::{{entity}Service, {entity}ServiceTrait},
    utils::service_tests,
};
use {project_lower}_db_interface::{entity_lowercase}::Mock{entity}DB;
use {project_lower}_services_interface::{entity_lowercase}::{
    Create{entity}Input, {entity}, Search{entity}Input, Update{entity}Input,
};

service_tests::simple_create_tests!(
    {entity}Service,
    [(), Mock{entity}DB],
    {entity},
    Create{entity}Input
);

service_tests::simple_get_by_id_tests!({entity}Service, Mock{entity}DB, {entity});

service_tests::simple_get_one_tests!(
    {entity}Service,
    [(), Mock{entity}DB],
    {entity},
    Search{entity}Input
);

service_tests::simple_update_tests!(
    {entity}Service,
    [(), Mock{entity}DB],
    {entity},
    Update{entity}Input
);

service_tests::simple_delete_by_id_tests!({entity}Service, Mock{entity}DB);
