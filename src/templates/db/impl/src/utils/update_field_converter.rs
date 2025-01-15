use maybe::Maybe;
use sea_orm::{
    sea_query::Nullable,
    ActiveValue::{self, NotSet, Set},
    Value as SeaOrmValue,
};

pub(crate) struct UpdateOptionFieldConverter<T> {
    field: Option<T>,
}

impl<T> From<Option<T>> for UpdateOptionFieldConverter<T> {
    fn from(field: Option<T>) -> Self {
        Self { field }
    }
}

impl<T> From<UpdateOptionFieldConverter<T>> for ActiveValue<T>
where
    T: Into<SeaOrmValue>,
{
    fn from(converter: UpdateOptionFieldConverter<T>) -> Self {
        converter.field.map_or(NotSet, |value| Set(value))
    }
}

pub(crate) struct UpdateMaybeFieldConverter<T> {
    field: Maybe<T>,
}

impl<T> From<Maybe<T>> for UpdateMaybeFieldConverter<T> {
    fn from(field: Maybe<T>) -> Self {
        Self { field }
    }
}

impl<T> From<UpdateMaybeFieldConverter<T>> for ActiveValue<Option<T>>
where
    T: Into<SeaOrmValue> + Nullable,
{
    fn from(converter: UpdateMaybeFieldConverter<T>) -> Self {
        match converter.field {
            Maybe::Some(value) => Set(Some(value)),
            Maybe::None => Set(None),
            Maybe::Void => NotSet,
        }
    }
}
