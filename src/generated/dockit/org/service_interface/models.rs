#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::org::{Org, UpdateOrgInput};
use dockit_common_models::{create_org_input, search_many_input, search_org_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_org_input! {}

search_org_input! {}

search_many_input!(SearchManyOrgsInput, SearchOrgInput);
