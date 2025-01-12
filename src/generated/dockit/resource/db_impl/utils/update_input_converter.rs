use crate::resource::entity::ActiveModel;
use db_models::resource::UpdateResourceInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateResourceInputConverter(UpdateResourceInput);

impl From<UpdateResourceInput> for UpdateResourceInputConverter {
    fn from(input: UpdateResourceInput) -> Self {
        Self(input)
    }
}

impl From<UpdateResourceInputConverter> for ActiveModel {
    fn from(converter: UpdateResourceInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			org_id: input.org_id.map_or(NotSet, |v| Set(v)),
			name: input.name.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
