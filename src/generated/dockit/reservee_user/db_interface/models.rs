use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ReserveeUser {
	pub id: Uuid,
	pub claw_auth_id: Uuid,
	pub email: String,
	pub description: String,
	pub created: NaiveDateTime,
	pub updated: NaiveDateTime,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateReserveeUserInput {
	pub id: Uuid,
	pub claw_auth_id: Uuid,
	pub email: String,
	pub description: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SearchReserveeUserInput {
	pub id: Option<SearchExactOperator<Uuid>>,
	pub claw_auth_id: Option<SearchExactOperator<Uuid>>,
	pub email: Option<SearchIterableOperator<String>>,
	pub description: Option<SearchIterableOperator<String>>,
	pub created: Option<SearchRangedOperator<NaiveDateTime>>,
	pub updated: Option<SearchRangedOperator<NaiveDateTime>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateReserveeUserInput {
    pub id: Uuid,
	pub claw_auth_id: Option<Uuid>,
	pub email: Option<String>,
	pub description: Option<String>,
}
