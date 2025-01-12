use crate::{
    reservee_user::{ReserveeUserService, ReserveeUserServiceTrait},
    utils::service_tests,
};
use dockit_db_interface::reservee_user::MockReserveeUserDB;
use dockit_services_interface::reservee_user::{
    CreateReserveeUserInput, ReserveeUser, SearchReserveeUserInput, UpdateReserveeUserInput,
};

service_tests::simple_create_tests!(
    ReserveeUserService,
    MockReserveeUserDB,
    ReserveeUser,
    CreateReserveeUserInput
);

service_tests::simple_get_by_id_tests!(ReserveeUserService, MockReserveeUserDB, ReserveeUser);

service_tests::simple_get_one_tests!(
    ReserveeUserService,
    MockReserveeUserDB,
    ReserveeUser,
    SearchReserveeUserInput
);

service_tests::simple_update_tests!(
    ReserveeUserService,
    MockReserveeUserDB,
    ReserveeUser,
    UpdateReserveeUserInput
);

service_tests::simple_delete_by_id_tests!(ReserveeUserService, MockReserveeUserDB);
