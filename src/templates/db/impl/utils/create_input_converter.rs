use crate::{entity_lowercase}::entity::ActiveModel;
use {project_lower}_db_models::{entity_lowercase}::Create{entity}Input;
use sea_orm::Set;
use uuid::Uuid;

pub(crate) struct Create{entity}InputConverter(Create{entity}Input);

impl From<Create{entity}Input> for Create{entity}InputConverter {
    fn from(input: Create{entity}Input) -> Self {
        Self(input)
    }
}

impl From<Create{entity}InputConverter> for ActiveModel {
    fn from(converter: Create{entity}InputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Set(Uuid::new_v4()),
{mapped_fields}
            ..Default::default()
        }
    }
}
