#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::reservee::{Reservee, UpdateReserveeInput};
use dockit_common_models::{create_reservee_input, search_many_input, search_reservee_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_reservee_input! {}

search_reservee_input! {}

search_many_input!(SearchManyReserveesInput, SearchReserveeInput);
