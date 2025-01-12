#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::org_user::{OrgUser, UpdateOrgUserInput};
use dockit_common_models::{create_org_user_input, search_many_input, search_org_user_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_org_user_input! {}

search_org_user_input! {}

search_many_input!(SearchManyOrgUsersInput, SearchOrgUserInput);
