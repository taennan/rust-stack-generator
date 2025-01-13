use crate::{entity_lowercase}::entity::ActiveModel;
use {project_lower}_db_models::{entity_lowercase}::Update{entity}Input;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct Update{entity}InputConverter(Update{entity}Input);

impl From<Update{entity}Input> for Update{entity}InputConverter {
    fn from(input: Update{entity}Input) -> Self {
        Self(input)
    }
}

impl From<Update{entity}InputConverter> for ActiveModel {
    fn from(converter: Update{entity}InputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
{mapped_fields}
            ..Default::default()
        }
    }
}
