#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::resource::{Resource, UpdateResourceInput};
use dockit_common_models::{create_resource_input, search_many_input, search_resource_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_resource_input! {}

search_resource_input! {}

search_many_input!(SearchManyResourcesInput, SearchResourceInput);
