use crate::{
    resource_reservation::{ResourceReservationService, ResourceReservationServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::resource_reservation::MockResourceReservationDB;
use dockit_services_interface::resource_reservation::{
    CreateResourceReservationInput, ResourceReservation, SearchResourceReservationInput, UpdateResourceReservationInput,
};

service_tests::simple_create_tests!(
    ResourceReservationService,
    MockResourceReservationDB,
    ResourceReservation,
    CreateResourceReservationInput
);

service_tests::simple_get_by_id_tests!(ResourceReservationService, MockResourceReservationDB, ResourceReservation);

service_tests::simple_get_one_tests!(
    ResourceReservationService,
    MockResourceReservationDB,
    ResourceReservation,
    SearchResourceReservationInput
);

service_tests::simple_update_tests!(
    ResourceReservationService,
    MockResourceReservationDB,
    ResourceReservation,
    UpdateResourceReservationInput
);

service_tests::simple_delete_by_id_tests!(ResourceReservationService, MockResourceReservationDB);
