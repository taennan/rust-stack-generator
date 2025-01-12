use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AdminApiKey {
	pub id: Uuid,
	pub hash: String,
	pub created: NaiveDateTime,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateAdminApiKeyInput {
	pub id: Uuid,
	pub hash: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SearchAdminApiKeyInput {
	pub id: Option<SearchExactOperator<Uuid>>,
	pub hash: Option<SearchIterableOperator<String>>,
	pub created: Option<SearchRangedOperator<NaiveDateTime>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateAdminApiKeyInput {
    pub id: Uuid,
	pub hash: Option<String>,
}
