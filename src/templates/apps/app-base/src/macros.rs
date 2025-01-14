#[macro_export]
macro_rules! app {
    () => {
        #[actix_web::main]
        pub async fn main() -> std::io::Result<()> {
            {project_lower}_app_base::main().await
        }
    };
}
