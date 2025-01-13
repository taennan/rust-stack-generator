use crate::search::SearchPrimitiveInput;
#[cfg(feature = "sea_orm")]
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr};
use uuid::Uuid;

macro_rules! search_exact_input {
    ($struct_name:ident, $value_type:ty) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::OneofObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $struct_name {
            #[cfg_attr(feature = "async_graphql", graphql(name = "EQUALS"))]
            Equals($value_type),
            #[cfg_attr(feature = "async_graphql", graphql(name = "NOT_EQUALS"))]
            NotEquals($value_type),
            #[cfg_attr(feature = "async_graphql", graphql(name = "IN"))]
            In(Vec<$value_type>),
            #[cfg_attr(feature = "async_graphql", graphql(name = "NOT_IN"))]
            NotIn(Vec<$value_type>),
        }

        impl SearchPrimitiveInput for $struct_name {
            #[cfg(feature = "sea_orm")]
            fn to_simple_expr<C>(self, column: C) -> SimpleExpr
            where
                C: ColumnTrait,
            {
                match self {
                    Self::Equals(v) => column.eq(v),
                    Self::NotEquals(v) => column.ne(v),
                    Self::In(vec) => column.is_in(vec),
                    Self::NotIn(vec) => column.is_not_in(vec),
                }
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::Equals(<$value_type>::default())
            }
        }
    };
}

search_exact_input!(SearchIdInput, Uuid);
