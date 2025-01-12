#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::resource_reservation_occupant::{ResourceReservationOccupant, UpdateResourceReservationOccupantInput};
use dockit_common_models::{create_resource_reservation_occupant_input, search_many_input, search_resource_reservation_occupant_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_resource_reservation_occupant_input! {}

search_resource_reservation_occupant_input! {}

search_many_input!(SearchManyResourceReservationOccupantsInput, SearchResourceReservationOccupantInput);
