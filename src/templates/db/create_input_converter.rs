use crate::{entity_name_lowercase}::entity::ActiveModel;
use db_models::{entity_name_lowercase}::Create{entity_name}Input;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct Create{entity_name}InputConverter(Create{entity_name}Input);

impl From<Create{entity_name}Input> for Create{entity_name}InputConverter {
    fn from(input: Create{entity_name}Input) -> Self {
        Self(input)
    }
}

impl From<Create{entity_name}InputConverter> for ActiveModel {
    fn from(converter: Create{entity_name}InputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
{mapped_fields}
            ..Default::default()
        }
    }
}
