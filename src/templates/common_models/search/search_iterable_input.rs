use crate::search::SearchPrimitiveInput;
#[cfg(feature = "sea_orm")]
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr};

macro_rules! search_iterable_input {
    ($struct_name:ident, $value_type:ty) => {
        #[cfg_attr(feature = "async_graphql", derive(async_graphql::OneofObject))]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $struct_name {
            #[cfg_attr(feature = "async_graphql", graphql(name = "EQUALS"))]
            Equals($value_type),
            #[cfg_attr(feature = "async_graphql", graphql(name = "NOT_EQUALS"))]
            NotEquals($value_type),
            #[cfg_attr(feature = "async_graphql", graphql(name = "HAS"))]
            Has($value_type),
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
                    Self::Has(v) => column.contains(v),
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

search_iterable_input!(SearchStringInput, String);
