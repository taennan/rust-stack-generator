use crate::{
    occupant::{OccupantService, OccupantServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::occupant::MockOccupantDB;
use dockit_services_interface::occupant::{
    CreateOccupantInput, Occupant, SearchOccupantInput, UpdateOccupantInput,
};

service_tests::simple_create_tests!(
    OccupantService,
    MockOccupantDB,
    Occupant,
    CreateOccupantInput
);

service_tests::simple_get_by_id_tests!(OccupantService, MockOccupantDB, Occupant);

service_tests::simple_get_one_tests!(
    OccupantService,
    MockOccupantDB,
    Occupant,
    SearchOccupantInput
);

service_tests::simple_update_tests!(
    OccupantService,
    MockOccupantDB,
    Occupant,
    UpdateOccupantInput
);

service_tests::simple_delete_by_id_tests!(OccupantService, MockOccupantDB);
