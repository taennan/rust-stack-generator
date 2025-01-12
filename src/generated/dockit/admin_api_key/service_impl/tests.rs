use crate::{
    admin_api_key::{AdminApiKeyService, AdminApiKeyServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::admin_api_key::MockAdminApiKeyDB;
use dockit_services_interface::admin_api_key::{
    CreateAdminApiKeyInput, AdminApiKey, SearchAdminApiKeyInput, UpdateAdminApiKeyInput,
};

service_tests::simple_create_tests!(
    AdminApiKeyService,
    MockAdminApiKeyDB,
    AdminApiKey,
    CreateAdminApiKeyInput
);

service_tests::simple_get_by_id_tests!(AdminApiKeyService, MockAdminApiKeyDB, AdminApiKey);

service_tests::simple_get_one_tests!(
    AdminApiKeyService,
    MockAdminApiKeyDB,
    AdminApiKey,
    SearchAdminApiKeyInput
);

service_tests::simple_update_tests!(
    AdminApiKeyService,
    MockAdminApiKeyDB,
    AdminApiKey,
    UpdateAdminApiKeyInput
);

service_tests::simple_delete_by_id_tests!(AdminApiKeyService, MockAdminApiKeyDB);
