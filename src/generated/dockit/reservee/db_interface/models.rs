use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Reservee {
	pub id: Uuid,
	pub org_id: Uuid,
	pub reservee_user_id: Option<Uuid>,
	pub first_name: Option<String>,
	pub middle_names: Option<String>,
	pub last_name: Option<String>,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub description: String,
	pub created: NaiveDateTime,
	pub updated: NaiveDateTime,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateReserveeInput {
	pub id: Uuid,
	pub org_id: Uuid,
	pub reservee_user_id: Option<Uuid>,
	pub first_name: Option<String>,
	pub middle_names: Option<String>,
	pub last_name: Option<String>,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub description: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SearchReserveeInput {
	pub id: Option<SearchExactOperator<Uuid>>,
	pub org_id: Option<SearchExactOperator<Uuid>>,
	pub reservee_user_id: Option<SearchExactOperator<Option<Uuid>>>,
	pub first_name: Option<SearchIterableOperator<Option<String>>>,
	pub middle_names: Option<SearchIterableOperator<Option<String>>>,
	pub last_name: Option<SearchIterableOperator<Option<String>>>,
	pub email: Option<SearchIterableOperator<Option<String>>>,
	pub phone: Option<SearchIterableOperator<Option<String>>>,
	pub description: Option<SearchIterableOperator<String>>,
	pub created: Option<SearchRangedOperator<NaiveDateTime>>,
	pub updated: Option<SearchRangedOperator<NaiveDateTime>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UpdateReserveeInput {
    pub id: Uuid,
	pub org_id: Option<Uuid>,
	pub reservee_user_id: Option<Option<Uuid>>,
	pub first_name: Option<Option<String>>,
	pub middle_names: Option<Option<String>>,
	pub last_name: Option<Option<String>>,
	pub email: Option<Option<String>>,
	pub phone: Option<Option<String>>,
	pub description: Option<String>,
}
