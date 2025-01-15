#[macro_export]
macro_rules! {entity_lower}_model {
    ($($field_name:ident: $field_type:ty,)*) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::SimpleObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct {entity} {
            pub id: uuid::Uuid,
            {model_fields}
            $(pub $field_name: $field_type),*
        }
    };
}

#[macro_export]
macro_rules! create_{entity_lower}_input {
    ($($field_name:ident: $field_type:ty,)*) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::InputObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct Create{entity}Input {
            {create_input_fields}
            $(pub $field_name: $field_type),*
        }
    };
}

#[macro_export]
macro_rules! search_{entity_lower}_input {
    ($($field_name:ident: $field_type:ty,)*) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::InputObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct Search{entity}Input {
            {search_input_fields}
            $(pub $field_name: $field_type),*
        }
    };
}

#[macro_export]
macro_rules! update_{entity_lower}_input {
    ($($field_name:ident: $field_type:ty,)*) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::InputObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct Update{entity}Input {
            pub id: uuid::Uuid,
            {update_input_fields}
            $(pub $field_name: $field_type),*
        }
    };
}

