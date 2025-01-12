#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::{entity_name_lowercase}::{{entity_name}, Update{entity_name}Input};
use dockit_common_models::{create_{entity_name_lowercase}_input, search_many_input, search_{entity_name_lowercase}_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_{entity_name_lowercase}_input! {}

search_{entity_name_lowercase}_input! {}

search_many_input!(SearchMany{entity_name}sInput, Search{entity_name}Input);
