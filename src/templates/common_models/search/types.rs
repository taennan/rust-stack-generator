#[cfg(feature = "sea_orm")]
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr};

pub trait SearchPrimitiveInput {
    #[cfg(feature = "sea_orm")]
    fn to_simple_expr<C>(self, column: C) -> SimpleExpr
    where
        C: ColumnTrait;
}
