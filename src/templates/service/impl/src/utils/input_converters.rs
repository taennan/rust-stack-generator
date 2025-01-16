macro_rules! struct_mapper {
    (
        $struct_name:ident,
        $from_struct:ty => $to_struct:ty,
        {
            $($mapped_field:ident),+$(,)?
        }
    ) => {
        pub(crate) struct $struct_name($from_struct);

        impl $struct_name {
            pub fn new(object: $from_struct) -> Self {
                Self(object)
            }
        }

        impl From<$from_struct> for $struct_name {
            fn from(object: $from_struct) -> Self {
                Self::new(object)
            }
        }

        impl From<$struct_name> for $to_struct {
            fn from(converter: $struct_name) -> Self {
                let object = converter.0;
                Self {
                    $($mapped_field: object.$mapped_field,)*
                }
            }
        }
    };
}

macro_rules! create_input_converter {
    (
        $struct_name:ident,
        $from_struct:ty => $to_struct:ty,
        {
            $($mapped_field:ident),+$(,)?
        }
    ) => {
        pub(crate) struct $struct_name {
            input: $from_struct,
            org_id: uuid::Uuid,
        }

        impl $struct_name {
            pub fn new(input: $from_struct, org_id: uuid::Uuid) -> Self {
                Self { input, org_id }
            }
        }

        impl From<$struct_name> for $to_struct {
            fn from(converter: $struct_name) -> Self {
                Self {
                    org_id: converter.org_id,
                    $($mapped_field: converter.input.$mapped_field,)*
                }
            }
        }
    };
}

macro_rules! search_input_converter {
    (
        $struct_name:ident,
        $from_struct:ty => $to_struct:ty,
        {
            $($mapped_field:ident),+$(,)?
        }
    ) => {
        pub(crate) struct $struct_name {
            input: $from_struct,
            org_id: uuid::Uuid,
        }

        impl $struct_name {
            pub fn new(input: $from_struct, org_id: uuid::Uuid) -> Self {
                Self { input, org_id }
            }
        }

        impl From<$struct_name> for $to_struct {
            fn from(converter: $struct_name) -> Self {
                Self {
                    org_id: Some(dockit_common_models::search::SearchIdInput::Equals(
                        converter.org_id,
                    )),
                    $($mapped_field: converter.input.$mapped_field,)*
                }
            }
        }
    };
}

macro_rules! search_many_input_converter {
    (
        $struct_name:ident,
        $from_struct:ty => $to_struct:ty,
        $search_one_converter:ident,
    ) => {
        pub(crate) struct $struct_name {
            input: $from_struct,
            org_id: uuid::Uuid,
        }

        impl $struct_name {
            pub fn new(input: $from_struct, org_id: uuid::Uuid) -> Self {
                Self { input, org_id }
            }
        }

        impl From<$struct_name> for $to_struct {
            fn from(converter: $struct_name) -> Self {
                let conditions_converter = $search_one_converter::new(
                    converter.input.conditions.unwrap_or_default(),
                    converter.org_id,
                );
                Self {
                    conditions: Some(conditions_converter.into()),
                    pagination: converter.input.pagination,
                }
            }
        }
    };
}

pub(crate) use create_input_converter;
pub(crate) use search_input_converter;
pub(crate) use search_many_input_converter;
pub(crate) use struct_mapper;
