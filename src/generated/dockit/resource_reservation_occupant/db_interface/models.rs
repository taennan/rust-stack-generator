use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceReservationOccupant {
	pub id: Uuid,
	pub org_id: Uuid,
	pub resource_reservation_id: Uuid,
	pub occupant_id: Uuid,
	pub created: NaiveDateTime,
	pub updated: NaiveDateTime,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateResourceReservationOccupantInput {
	pub id: Uuid,
	pub org_id: Uuid,
	pub resource_reservation_id: Uuid,
	pub occupant_id: Uuid,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SearchResourceReservationOccupantInput {
	pub id: Option<SearchExactOperator<Uuid>>,
	pub org_id: Option<SearchExactOperator<Uuid>>,
	pub resource_reservation_id: Option<SearchExactOperator<Uuid>>,
	pub occupant_id: Option<SearchExactOperator<Uuid>>,
	pub created: Option<SearchRangedOperator<NaiveDateTime>>,
	pub updated: Option<SearchRangedOperator<NaiveDateTime>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateResourceReservationOccupantInput {
    pub id: Uuid,
	pub org_id: Option<Uuid>,
	pub resource_reservation_id: Option<Uuid>,
	pub occupant_id: Option<Uuid>,
}
