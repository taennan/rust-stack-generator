use {project_lower}_config;
#[cfg(feature = "{project_lower}_graphql")]
use {project_lower}_graphql::{{project}Data, {project}Schema};

#[derive(Clone)]
pub(crate) struct AppData {
    #[cfg(feature = "{project_lower}_graphql")]
    {project_lower}_schema: {project}Schema,
}

impl AppData {
    pub async fn new() -> Self {
        Self {
            #[cfg(feature = "{project_lower}_graphql")]
            {project_lower}_schema: {project}Schema::from(
                {project}Data::new(
                    &{project_lower}_config::db::uri(),
                    {project_lower}_config::auth::claw_auth_api_key(),
                    {project_lower}_config::auth::mock_claw_auth(),
                )
                .await,
            ),
        }
    }

    #[cfg(feature = "{project_lower}_graphql")]
    pub fn {project_lower}_schema(&self) -> {project}Schema {
        self.{project_lower}_schema.clone()
    }
}
