use crate::{
    resource::{ResourceService, ResourceServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::resource::MockResourceDB;
use dockit_services_interface::resource::{
    CreateResourceInput, Resource, SearchResourceInput, UpdateResourceInput,
};

service_tests::simple_create_tests!(
    ResourceService,
    MockResourceDB,
    Resource,
    CreateResourceInput
);

service_tests::simple_get_by_id_tests!(ResourceService, MockResourceDB, Resource);

service_tests::simple_get_one_tests!(
    ResourceService,
    MockResourceDB,
    Resource,
    SearchResourceInput
);

service_tests::simple_update_tests!(
    ResourceService,
    MockResourceDB,
    Resource,
    UpdateResourceInput
);

service_tests::simple_delete_by_id_tests!(ResourceService, MockResourceDB);
