use crate::{entity_name_lowercase}::entity::ActiveModel;
use db_models::{entity_name_lowercase}::Update{entity_name}Input;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct Update{entity_name}InputConverter(Update{entity_name}Input);

impl From<Update{entity_name}Input> for Update{entity_name}InputConverter {
    fn from(input: Update{entity_name}Input) -> Self {
        Self(input)
    }
}

impl From<Update{entity_name}InputConverter> for ActiveModel {
    fn from(converter: Update{entity_name}InputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
{mapped_fields}
            ..Default::default()
        }
    }
}
