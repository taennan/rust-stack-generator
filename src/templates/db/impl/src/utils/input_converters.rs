macro_rules! create_input_converter {
    (
        $struct_name:ident,
        $from_struct:ty => $to_struct:ty,
        {
            $($mapped_field:ident),$(,)?
        }
    ) => {
        pub(crate) struct $struct_name($from_struct);

        impl From<$from_struct> for $struct_name {
            fn from(input: $from_struct) -> Self {
                Self(input)
            }
        }

        impl From<$struct_name> for $to_struct {
            fn from(converter: $struct_name) -> Self {
                let input = converter.0;
                Self {
                    id: sea_orm::Set(uuid::Uuid::new_v4()),
                    $($mapped_field: sea_orm::Set(input.$mapped_field),)*
                    ..Default::default()
                }
            }
        }
    };
}

macro_rules! struct_from_mapper {
    (
        $from_struct:ty => $to_struct:ty,
        {
            $($mapped_field:ident),+$(,)?
        }
    ) => {
        impl From<$from_struct> for $to_struct {
            fn from(object: $from_struct) -> Self {
                Self {
                    $($mapped_field: object.$mapped_field,)*
                }
            }
        }
    };
}

macro_rules! search_input_converter {
    (
        $struct_name:ident,
        $from_struct:ty => $to_struct:ident,
        {
            $(($mapped_field:ident, $column:expr, $converter_method:ident)),+$(,)?
        }
    ) => {
        pub(crate) struct $struct_name($from_struct);

        impl From<$from_struct> for $struct_name {
            fn from(input: $from_struct) -> Self {
                Self(input)
            }
        }

        impl From<Option<$from_struct>> for $struct_name {
            fn from(input: Option<$from_struct>) -> Self {
                Self(input.unwrap_or_default())
            }
        }

        impl From<$struct_name> for sea_orm::Select<$to_struct> {
            fn from(converter: $struct_name) -> Self {
                use sea_orm::{QueryTrait, entity::prelude::*};

                let input = converter.0;

                $to_struct::find()
                    $(
                        .apply_if(
                            input.$mapped_field,
                            crate::search::SearchFieldConverter::new($column).$converter_method()
                        )
                    )*
            }
        }
    };
}

macro_rules! update_input_converter {
    (
        $struct_name:ident,
        $from_struct:ty => $to_struct:ident,
        {
            $($mapped_option_field:ident),*$(,)?
        },
        {
            $($mapped_maybe_field:ident),*$(,)?
        }
    ) => {
        pub(crate) struct $struct_name($from_struct);

        impl From<$from_struct> for $struct_name {
            fn from(input: $from_struct) -> Self {
                Self(input)
            }
        }

        impl From<$struct_name> for $to_struct {
            fn from(converter: $struct_name) -> Self {
                let input = converter.0;
                Self {
                    id: sea_orm::Unchanged(input.id),
                    $($mapped_option_field: crate::utils::UpdateOptionFieldConverter::from(input.$mapped_option_field).into(),)*
                    $($mapped_maybe_field: crate::utils::UpdateMaybeFieldConverter::from(input.$mapped_maybe_field).into(),)*
                    ..Default::default()
                }
            }
        }
    };
}

pub(crate) use create_input_converter;
pub(crate) use search_input_converter;
pub(crate) use struct_from_mapper;
pub(crate) use update_input_converter;
