#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
use uuid::Uuid;

#[macro_export]
macro_rules! update_relation_input {
    ($struct_name:ident, $create_input:ty, $update_input:ty) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::OneofObject))]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $struct_name {
            Create($create_input),
            Delete(uuid::Uuid),
            Replace({project_lower}_common_models::update::ReplaceByIdInput),
            Update($update_input),
        }

        impl $struct_name {
            pub fn is_delete(&self) -> bool {
                matches!(self, Self::Delete(_))
            }
        }
    };
}

#[cfg_attr(feature = "async_graphql", derive(InputObject))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReplaceByIdInput {
    pub new_id: Uuid,
    pub old_id: Uuid,
}

impl ReplaceByIdInput {
    pub fn new(new_id: Uuid, old_id: Uuid) -> Self {
        Self { new_id, old_id }
    }
}

#[macro_export]
macro_rules! update_iterable_input {
    ($struct_name:ident, $value_type:ty) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::OneofObject))]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $struct_name {
            Append(Vec<$value_type>),
            Prepend(Vec<$value_type>),
            Remove(Vec<$value_type>),
            Set(Vec<$value_type>),
        }
    };
}
