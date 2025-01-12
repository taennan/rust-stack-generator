use crate::{
    resource_reservation_occupant::{ResourceReservationOccupantService, ResourceReservationOccupantServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::resource_reservation_occupant::MockResourceReservationOccupantDB;
use dockit_services_interface::resource_reservation_occupant::{
    CreateResourceReservationOccupantInput, ResourceReservationOccupant, SearchResourceReservationOccupantInput, UpdateResourceReservationOccupantInput,
};

service_tests::simple_create_tests!(
    ResourceReservationOccupantService,
    MockResourceReservationOccupantDB,
    ResourceReservationOccupant,
    CreateResourceReservationOccupantInput
);

service_tests::simple_get_by_id_tests!(ResourceReservationOccupantService, MockResourceReservationOccupantDB, ResourceReservationOccupant);

service_tests::simple_get_one_tests!(
    ResourceReservationOccupantService,
    MockResourceReservationOccupantDB,
    ResourceReservationOccupant,
    SearchResourceReservationOccupantInput
);

service_tests::simple_update_tests!(
    ResourceReservationOccupantService,
    MockResourceReservationOccupantDB,
    ResourceReservationOccupant,
    UpdateResourceReservationOccupantInput
);

service_tests::simple_delete_by_id_tests!(ResourceReservationOccupantService, MockResourceReservationOccupantDB);
