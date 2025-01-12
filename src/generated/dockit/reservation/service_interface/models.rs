#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::reservation::{Reservation, UpdateReservationInput};
use dockit_common_models::{create_reservation_input, search_many_input, search_reservation_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_reservation_input! {}

search_reservation_input! {}

search_many_input!(SearchManyReservationsInput, SearchReservationInput);
