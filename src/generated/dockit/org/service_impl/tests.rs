use crate::{
    org::{OrgService, OrgServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::org::MockOrgDB;
use dockit_services_interface::org::{
    CreateOrgInput, Org, SearchOrgInput, UpdateOrgInput,
};

service_tests::simple_create_tests!(
    OrgService,
    MockOrgDB,
    Org,
    CreateOrgInput
);

service_tests::simple_get_by_id_tests!(OrgService, MockOrgDB, Org);

service_tests::simple_get_one_tests!(
    OrgService,
    MockOrgDB,
    Org,
    SearchOrgInput
);

service_tests::simple_update_tests!(
    OrgService,
    MockOrgDB,
    Org,
    UpdateOrgInput
);

service_tests::simple_delete_by_id_tests!(OrgService, MockOrgDB);
