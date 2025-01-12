use crate::{
    org_user::{OrgUserService, OrgUserServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::org_user::MockOrgUserDB;
use dockit_services_interface::org_user::{
    CreateOrgUserInput, OrgUser, SearchOrgUserInput, UpdateOrgUserInput,
};

service_tests::simple_create_tests!(
    OrgUserService,
    MockOrgUserDB,
    OrgUser,
    CreateOrgUserInput
);

service_tests::simple_get_by_id_tests!(OrgUserService, MockOrgUserDB, OrgUser);

service_tests::simple_get_one_tests!(
    OrgUserService,
    MockOrgUserDB,
    OrgUser,
    SearchOrgUserInput
);

service_tests::simple_update_tests!(
    OrgUserService,
    MockOrgUserDB,
    OrgUser,
    UpdateOrgUserInput
);

service_tests::simple_delete_by_id_tests!(OrgUserService, MockOrgUserDB);
