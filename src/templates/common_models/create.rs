#[macro_export]
macro_rules! create_relation_input {
    ($struct_name:ident, $create_input:ty) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::OneofObject))]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $struct_name {
            Create($create_input),
            Retrieve(uuid::Uuid),
        }

        #[cfg(test)]
        impl Default for $struct_name {
            fn default() -> Self {
                Self::Retrieve(uuid::Uuid::default())
            }
        }
    };
}
