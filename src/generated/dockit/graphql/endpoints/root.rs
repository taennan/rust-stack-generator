use crate::{
    endpoints::{
	admin_api_key::{AdminApiKeyQueries, AdminApiKeyMutations},
	occupant::{OccupantQueries, OccupantMutations},
	org::{OrgQueries, OrgMutations},
	org_user::{OrgUserQueries, OrgUserMutations},
	reservation::{ReservationQueries, ReservationMutations},
	reservee::{ReserveeQueries, ReserveeMutations},
	reservee_user::{ReserveeUserQueries, ReserveeUserMutations},
	resource::{ResourceQueries, ResourceMutations},
	resource_reservation::{ResourceReservationQueries, ResourceReservationMutations},
	resource_reservation_occupant::{ResourceReservationOccupantQueries, ResourceReservationOccupantMutations},},
    utils::AuthGuard,
};
use actix_web::{
    guard,
    web::{self, Data},
    HttpRequest, HttpResponse, Resource,
};
use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[derive(MergedObject, Default)]
pub struct RootQueries(
	AdminApiKeyQuery,
	OccupantQuery,
	OrgQuery,
	OrgUserQuery,
	ReservationQuery,
	ReserveeQuery,
	ReserveeUserQuery,
	ResourceQuery,
	ResourceReservationQuery,
	ResourceReservationOccupantQuery,

);

#[derive(MergedObject, Default)]
pub struct RootMutations(
	AdminApiKeyMutation,
	OccupantMutation,
	OrgMutation,
	OrgUserMutation,
	ReservationMutation,
	ReserveeMutation,
	ReserveeUserMutation,
	ResourceMutation,
	ResourceReservationMutation,
	ResourceReservationOccupantMutation,

);

pub type RootSchema = Schema<RootQueries, RootMutations, EmptySubscription>;

pub fn index() -> Resource {
    web::resource("/graphql")
        .guard(guard::Post())
        .to(index_impl)
}

async fn index_impl(
    schema: Data<RootSchema>,
    http_request: HttpRequest,
    graphql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = graphql_request.into_inner();
    let auth = AuthGuard::try_from(&http_request);
    if let Ok(auth) = auth {
        let claims = auth.claims().clone();
        request = request.data(claims);
    }
    schema.execute(request).await.into()
}

pub fn playground() -> Resource {
    web::resource("/graphql")
        .guard(guard::Get())
        .to(playground_impl)
}

async fn playground_impl() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/ws")
                .finish(),
        )
}
