use {project_lower}_common_models::search::{
    SearchDateTimeInput, SearchIdInput, SearchPrimitiveInput
    SearchStringInput,
};
use sea_orm::{ColumnTrait, QueryFilter};
use std::marker::PhantomData;

pub(crate) struct SearchFieldConverter<S, C> {
    column: C,
    select: PhantomData<S>,
}

impl<S, C> SearchFieldConverter<S, C>
where
    S: QueryFilter,
    C: ColumnTrait,
{
    pub fn new(column: C) -> Self {
        Self {
            select: PhantomData,
            column,
        }
    }

    //
    // Exact Inputs
    //

    pub fn id(self) -> impl (FnOnce(S, SearchIdInput) -> S) {
        move |select, input| select.filter(input.to_simple_expr(self.column))
    }

    //
    // Ranged Inputs
    //

    pub fn int(self) -> impl (FnOnce(S, SearchIntInput) -> S) {
        move |select, input| select.filter(input.to_simple_expr(self.column))
    }

    pub fn date_time(self) -> impl (FnOnce(S, SearchDateTimeInput) -> S) {
        move |select, input| select.filter(input.to_simple_expr(self.column))
    }

    //
    // Iterable Inputs
    //

    pub fn string(self) -> impl (FnOnce(S, SearchStringInput) -> S) {
        move |select, input| select.filter(input.to_simple_expr(self.column))
    }
}
