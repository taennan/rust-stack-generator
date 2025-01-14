use crate::app_data::AppData;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use {project_lower}_config;
#[cfg(feature = "{project_lower}_debug_rest")]
use {project_lower}_debug_rest;
#[cfg(feature = "{project_lower}_graphql")]
use {project_lower}_graphql;
use dotenv::dotenv;
use env_logger::Env;

pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let app_data = AppData::new().await;

    HttpServer::new(move || {
        let mut app = App::new().wrap(Logger::default()).wrap(Cors::permissive());

        #[cfg(feature = "{project_lower}_graphql")]
        {
            let gql_route = &{project_lower}_config::api::gql_route();
            app = app.app_data(Data::new(app_data.{project_lower}_schema()));
            app = app.service({project_lower}_graphql::routes::index(gql_route));
            app = app.service({project_lower}_graphql::routes::playground(gql_route));
        }

        #[cfg(feature = "{project_lower}_admin_graphql")]
        {}

        app
    })
    .bind(({project_lower}_config::api::host(), {project_lower}_config::api::port()))?
    .run()
    .await
}
