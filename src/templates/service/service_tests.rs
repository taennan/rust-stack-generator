use crate::{
    {entity_name_lowercase}::{{entity_name}Service, {entity_name}ServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::{entity_name_lowercase}::Mock{entity_name}DB;
use dockit_services_interface::{entity_name_lowercase}::{
    Create{entity_name}Input, {entity_name}, Search{entity_name}Input, Update{entity_name}Input,
};

service_tests::simple_create_tests!(
    {entity_name}Service,
    Mock{entity_name}DB,
    {entity_name},
    Create{entity_name}Input
);

service_tests::simple_get_by_id_tests!({entity_name}Service, Mock{entity_name}DB, {entity_name});

service_tests::simple_get_one_tests!(
    {entity_name}Service,
    Mock{entity_name}DB,
    {entity_name},
    Search{entity_name}Input
);

service_tests::simple_update_tests!(
    {entity_name}Service,
    Mock{entity_name}DB,
    {entity_name},
    Update{entity_name}Input
);

service_tests::simple_delete_by_id_tests!({entity_name}Service, Mock{entity_name}DB);
