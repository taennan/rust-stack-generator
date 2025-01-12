use crate::{
    reservation::{ReservationService, ReservationServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::reservation::MockReservationDB;
use dockit_services_interface::reservation::{
    CreateReservationInput, Reservation, SearchReservationInput, UpdateReservationInput,
};

service_tests::simple_create_tests!(
    ReservationService,
    MockReservationDB,
    Reservation,
    CreateReservationInput
);

service_tests::simple_get_by_id_tests!(ReservationService, MockReservationDB, Reservation);

service_tests::simple_get_one_tests!(
    ReservationService,
    MockReservationDB,
    Reservation,
    SearchReservationInput
);

service_tests::simple_update_tests!(
    ReservationService,
    MockReservationDB,
    Reservation,
    UpdateReservationInput
);

service_tests::simple_delete_by_id_tests!(ReservationService, MockReservationDB);
