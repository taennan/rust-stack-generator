use crate::{
    reservee::{ReserveeService, ReserveeServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::reservee::MockReserveeDB;
use dockit_services_interface::reservee::{
    CreateReserveeInput, Reservee, SearchReserveeInput, UpdateReserveeInput,
};

service_tests::simple_create_tests!(
    ReserveeService,
    MockReserveeDB,
    Reservee,
    CreateReserveeInput
);

service_tests::simple_get_by_id_tests!(ReserveeService, MockReserveeDB, Reservee);

service_tests::simple_get_one_tests!(
    ReserveeService,
    MockReserveeDB,
    Reservee,
    SearchReserveeInput
);

service_tests::simple_update_tests!(
    ReserveeService,
    MockReserveeDB,
    Reservee,
    UpdateReserveeInput
);

service_tests::simple_delete_by_id_tests!(ReserveeService, MockReserveeDB);
