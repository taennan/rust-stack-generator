use crate::org::entity::ActiveModel;
use db_models::org::UpdateOrgInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateOrgInputConverter(UpdateOrgInput);

impl From<UpdateOrgInput> for UpdateOrgInputConverter {
    fn from(input: UpdateOrgInput) -> Self {
        Self(input)
    }
}

impl From<UpdateOrgInputConverter> for ActiveModel {
    fn from(converter: UpdateOrgInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			name: input.name.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
