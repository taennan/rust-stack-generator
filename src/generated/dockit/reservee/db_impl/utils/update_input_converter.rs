use crate::reservee::entity::ActiveModel;
use db_models::reservee::UpdateReserveeInput;
use sea_orm::{NotSet, Set, Unchanged};

pub(crate) struct UpdateReserveeInputConverter(UpdateReserveeInput);

impl From<UpdateReserveeInput> for UpdateReserveeInputConverter {
    fn from(input: UpdateReserveeInput) -> Self {
        Self(input)
    }
}

impl From<UpdateReserveeInputConverter> for ActiveModel {
    fn from(converter: UpdateReserveeInputConverter) -> Self {
        let input = converter.0;
        Self {
            id: Unchanged(input.id),
			org_id: input.org_id.map_or(NotSet, |v| Set(v)),
			reservee_user_id: input.reservee_user_id.map_or(NotSet, |v| Set(v)),
			first_name: input.first_name.map_or(NotSet, |v| Set(v)),
			middle_names: input.middle_names.map_or(NotSet, |v| Set(v)),
			last_name: input.last_name.map_or(NotSet, |v| Set(v)),
			email: input.email.map_or(NotSet, |v| Set(v)),
			phone: input.phone.map_or(NotSet, |v| Set(v)),
			description: input.description.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
