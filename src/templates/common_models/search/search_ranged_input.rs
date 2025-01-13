use crate::search::SearchPrimitiveInput;
use chrono::NaiveDateTime;
#[cfg(feature = "sea_orm")]
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr};

macro_rules! search_ranged_input {
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
            #[cfg_attr(feature = "async_graphql", graphql(name = "GREATER_THAN"))]
            GreaterThan($value_type),
            #[cfg_attr(feature = "async_graphql", graphql(name = "LESS_THAN"))]
            LessThan($value_type),
            #[cfg_attr(feature = "async_graphql", graphql(name = "GREATER_THAN_OR_EQUAL"))]
            GreaterThanOrEqual($value_type),
            #[cfg_attr(feature = "async_graphql", graphql(name = "LESS_THAN_OR_EQUAL"))]
            LessThanOrEqual($value_type),
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
                    Self::GreaterThan(v) => column.gt(v),
                    Self::LessThan(v) => column.lt(v),
                    Self::GreaterThanOrEqual(v) => column.gte(v),
                    Self::LessThanOrEqual(v) => column.lte(v),
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

search_ranged_input!(SearchUIntInput, u32);
search_ranged_input!(SearchDateTimeInput, NaiveDateTime);
