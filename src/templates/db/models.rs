use chrono::NaiveDateTime;
use search_operator::{SearchExactOperator, SearchIterableOperator, SearchRangedOperator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct {entity_name} {
{model_fields}
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Create{entity_name}Input {
{create_input_fields}
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Search{entity_name}Input {
{search_input_fields}
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Update{entity_name}Input {
    pub id: Uuid,
{update_input_fields}
}
