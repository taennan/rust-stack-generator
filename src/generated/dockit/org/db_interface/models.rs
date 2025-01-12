use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Org {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub created: NaiveDateTime,
	pub updated: NaiveDateTime,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateOrgInput {
	pub id: Uuid,
	pub name: String,
	pub description: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SearchOrgInput {
	pub id: Option<SearchExactOperator<Uuid>>,
	pub name: Option<SearchIterableOperator<String>>,
	pub description: Option<SearchIterableOperator<String>>,
	pub created: Option<SearchRangedOperator<NaiveDateTime>>,
	pub updated: Option<SearchRangedOperator<NaiveDateTime>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateOrgInput {
    pub id: Uuid,
	pub name: Option<String>,
	pub description: Option<String>,
}
