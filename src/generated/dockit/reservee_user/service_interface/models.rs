#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::reservee_user::{ReserveeUser, UpdateReserveeUserInput};
use dockit_common_models::{create_reservee_user_input, search_many_input, search_reservee_user_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_reservee_user_input! {}

search_reservee_user_input! {}

search_many_input!(SearchManyReserveeUsersInput, SearchReserveeUserInput);
