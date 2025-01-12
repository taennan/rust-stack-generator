#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::resource_reservation::{ResourceReservation, UpdateResourceReservationInput};
use dockit_common_models::{create_resource_reservation_input, search_many_input, search_resource_reservation_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_resource_reservation_input! {}

search_resource_reservation_input! {}

search_many_input!(SearchManyResourceReservationsInput, SearchResourceReservationInput);
