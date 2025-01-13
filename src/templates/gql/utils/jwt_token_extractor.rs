use actix_web::HttpRequest;
use {project_lower}_error::{project_prefix}Error;

#[derive(Clone, Debug)]
pub struct JwtTokenExtractor {
    jwt: String,
}

impl JwtTokenExtractor {
    fn new(jwt: String) -> Self {
        Self { jwt }
    }

    pub fn jwt(&self) -> String {
        self.jwt.clone()
    }
}

impl TryFrom<&HttpRequest> for JwtTokenExtractor {
    type Error = {project_prefix}Error;

    fn try_from(request: &HttpRequest) -> Result<Self, Self::Error> {
        let auth_header = request.headers().get("Authorization");
        if auth_header.is_none() {
            let error = {project_prefix}Error::Unauthorized("No authorization header found".to_string());
            return Err(error);
        }

        let auth_header = auth_header.unwrap().to_str();
        if let Err(_) = auth_header {
            let error = make_error("Authorization header should only contain ASCII characters");
            return Err(error);
        }

        let splits: Vec<&str> = auth_header.unwrap().split(" ").collect();
        let bearer_split = splits.get(0);
        let jwt_split = splits.get(1);

        if bearer_split.is_none()
            || *(bearer_split.unwrap()) != "Bearer"
            || jwt_split.is_none()
            || jwt_split.unwrap().trim_matches(' ') == ""
        {
            let error = make_error(
                "Authorization header is formatted incorrectly, expected 'Bearer <jwt_token>'",
            );
            return Err(error);
        }

        let jwt_token = jwt_split.unwrap().to_string();
        Ok(Self::new(jwt_token))
    }
}

fn make_error(error_message: &str) -> {project_prefix}Error {
    eprintln!("{error_message}");
    {project_prefix}Error::Unauthorized(error_message.to_string())
}
