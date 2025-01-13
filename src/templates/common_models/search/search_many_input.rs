#[macro_export]
macro_rules! search_many_input {
    ($struct_name:ident, $condition_type:ty) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::InputObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $struct_name {
            pub conditions: Option<$condition_type>,
            pub pagination: Option<{project_lower}_common_models::search::PaginationInput>,
        }
    };
    ($struct_name:ident, $condition_type:ty, $pagination_type:ty) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::InputObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $struct_name {
            pub conditions: Option<$condition_type>,
            pub pagination: Option<$pagination_type>,
        }
    };
}
