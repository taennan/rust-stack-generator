use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Reservation {
	pub id: Uuid,
	pub org_id: Uuid,
	pub reservee_id: Uuid,
	pub status: String,
	pub description: String,
	pub created: NaiveDateTime,
	pub updated: NaiveDateTime,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateReservationInput {
	pub id: Uuid,
	pub org_id: Uuid,
	pub reservee_id: Uuid,
	pub status: String,
	pub description: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SearchReservationInput {
	pub id: Option<SearchExactOperator<Uuid>>,
	pub org_id: Option<SearchExactOperator<Uuid>>,
	pub reservee_id: Option<SearchExactOperator<Uuid>>,
	pub status: Option<SearchIterableOperator<String>>,
	pub description: Option<SearchIterableOperator<String>>,
	pub created: Option<SearchRangedOperator<NaiveDateTime>>,
	pub updated: Option<SearchRangedOperator<NaiveDateTime>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateReservationInput {
    pub id: Uuid,
	pub org_id: Option<Uuid>,
	pub reservee_id: Option<Uuid>,
	pub status: Option<String>,
	pub description: Option<String>,
}
