#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::occupant::{Occupant, UpdateOccupantInput};
use dockit_common_models::{create_occupant_input, search_many_input, search_occupant_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_occupant_input! {}

search_occupant_input! {}

search_many_input!(SearchManyOccupantsInput, SearchOccupantInput);
