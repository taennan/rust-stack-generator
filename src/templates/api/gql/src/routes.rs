use crate::{utils::JwtTokenExtractor, {project_prefix}Schema};
use actix_web::{
    guard,
    web::{self, Data},
    HttpRequest, HttpResponse, Resource,
};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub fn index(route: &str) -> Resource {
    web::resource(route).guard(guard::Post()).to(index_impl)
}

async fn index_impl(
    schema: Data<{project_prefix}Schema>,
    http_request: HttpRequest,
    graphql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = graphql_request.into_inner();

    let extractor_result = JwtTokenExtractor::try_from(&http_request);
    if let Ok(extractor) = extractor_result {
        request = request.data(extractor);
    }

    schema.execute(request).await.into()
}

pub fn playground(route: &str) -> Resource {
    web::resource(route).guard(guard::Get()).to(playground_impl)
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
