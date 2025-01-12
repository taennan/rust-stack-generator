use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceReservation {
	pub id: Uuid,
	pub org_id: Uuid,
	pub resource_id: Uuid,
	pub reservation_id: Uuid,
	pub description: String,
	pub created: NaiveDateTime,
	pub updated: NaiveDateTime,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateResourceReservationInput {
	pub id: Uuid,
	pub org_id: Uuid,
	pub resource_id: Uuid,
	pub reservation_id: Uuid,
	pub description: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SearchResourceReservationInput {
	pub id: Option<SearchExactOperator<Uuid>>,
	pub org_id: Option<SearchExactOperator<Uuid>>,
	pub resource_id: Option<SearchExactOperator<Uuid>>,
	pub reservation_id: Option<SearchExactOperator<Uuid>>,
	pub description: Option<SearchIterableOperator<String>>,
	pub created: Option<SearchRangedOperator<NaiveDateTime>>,
	pub updated: Option<SearchRangedOperator<NaiveDateTime>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateResourceReservationInput {
    pub id: Uuid,
	pub org_id: Option<Uuid>,
	pub resource_id: Option<Uuid>,
	pub reservation_id: Option<Uuid>,
	pub description: Option<String>,
}
