/* In case we need more customized endpoints
macro_rules! endpoint_fn {
    ($endpoint_name:ident($($arg_name:ident: $arg_type:ty),*) -> $return_type:ident {
        $service_getter:ident$(.$awaiter:tt)?
        $service_method:ident
    }) => {
        pub async fn $endpoint_name(
            &self,
            ctx: &async_graphql::Context<'_>,
            $($arg_name: $arg_type,)*
        ) -> async_graphql::FieldResult<Occupant> {
            let output = crate::context::GQLContextWrapper::from(ctx)
                .$service_getter()
                $(.$awaiter?)?
                .$service_method($($arg_name,)*)
                .await?;
            Ok(output)
        }
    };
}
*/

macro_rules! gql_endpoints {
    (
        $object_name:ident,
        $(
            $endpoint_name:ident($($arg_name:ident: $arg_type:ty),*) -> $return_type:ty {
                $service_getter:ident$(.$awaiter:tt)?
                $service_method:ident
            }
        )*
    ) => {
        #[derive(Default)]
        pub struct $object_name;

        #[async_graphql::Object]
        impl $object_name {
            $(
                pub async fn $endpoint_name(
                    &self,
                    ctx: &async_graphql::Context<'_>,
                    $($arg_name: $arg_type,)*
                ) -> async_graphql::FieldResult<$return_type> {
                    let output = crate::context::GQLContextWrapper::from(ctx)
                        .$service_getter()
                        $(.$awaiter?)?
                        .$service_method($($arg_name,)*)
                        .await?;
                    Ok(output)
                }
            )*
        }
    };
}

//pub(crate) use endpoint_fn;
pub(crate) use gql_endpoints;
