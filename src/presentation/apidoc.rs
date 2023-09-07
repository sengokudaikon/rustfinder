use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_auto_discovery::utoipa_auto_discovery;
use crate::application::auth::login_cqrs::{LoginCommand, LoginResponse, SignUpCommand, SignUpResponse};
use crate::presentation::auth::{login, register};
#[utoipa_auto_discovery(
    paths = "( crate::application::auth::login_cqrs => ./src/application/auth/login_cqrs.rs)"
)]
#[derive(OpenApi)]
#[openapi(
info(description = "API Documentation"),
paths(crate::presentation::auth::register, crate::presentation::auth::login),
components(schemas(LoginCommand, LoginResponse, SignUpCommand, SignUpResponse)),
modifiers(&SecurityAddon)
)]
pub(crate) struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        )
    }
}