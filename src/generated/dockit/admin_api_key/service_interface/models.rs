#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
pub use dockit_common_models::entities::admin_api_key::{AdminApiKey, UpdateAdminApiKeyInput};
use dockit_common_models::{create_admin_api_key_input, search_many_input, search_admin_api_key_input};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

create_admin_api_key_input! {}

search_admin_api_key_input! {}

search_many_input!(SearchManyAdminApiKeysInput, SearchAdminApiKeyInput);
