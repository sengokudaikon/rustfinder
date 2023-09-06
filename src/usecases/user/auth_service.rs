use std::sync::Arc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::http::Status;
use argon2::{self, Argon2};
use async_trait::async_trait;
use password_hash::{PasswordHash, PasswordVerifier};
use crate::application::auth::login_cqrs::{LoginCommand, LoginResponse};
use crate::domain::user::entity::user::{User, Claims};
use crate::domain::user::repository::port::UserRepositoryPort;
use crate::persistence::user::repository::user_repository::UserRepository;

#[async_trait]
pub(crate) trait AuthServicePort {
    async fn authenticate_user(&self, login_request: LoginCommand) -> Result<LoginResponse, Status>;
    async fn validate_password(&self, password: &str, saved_hash: &str) -> bool;
    async fn encode_jwt(&self, user: &User, expiration: usize) -> Result<String, jsonwebtoken::errors::Error>;
    async fn decode_jwt(&self, token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error>;
}
pub(crate) struct AuthService {
    pub user_repository: Arc<UserRepository>,
}

impl AuthService {
    pub(crate) async fn new() -> AuthService {
        AuthService {user_repository: Arc::new(UserRepository::new(
            crate::infrastructure::database::connection::establish_connection().await
        ))}
    }
}

#[async_trait]
impl AuthServicePort for AuthService {
    async fn authenticate_user(&self, login_request: LoginCommand) -> Result<LoginResponse, Status> {
        match self.user_repository.find_user_by_email(&login_request.email).await {
            Ok(user) => {
                if self.validate_password(&login_request.password, &user.password).await {
                    match self.encode_jwt(&user, 3600).await {
                        Ok(token) => Ok(LoginResponse { jwt: token.to_string(),}),
                        Err(_) => Err(Status::InternalServerError),
                    }
                } else {
                    Err(Status::Unauthorized)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }

    async fn validate_password(&self, password: &str, hash: &str) -> bool {
        let password_hash = PasswordHash::new(hash).expect("Invalid hash");
        let password_verifier = Argon2::default();

        password_verifier
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok()
    }

    async fn encode_jwt(&self, user: &User, expiration: usize) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = dotenv::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration,
            roles: user.roles.clone(),
        };
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        encode(&header, &claims, &encoding_key)
    }

    async fn decode_jwt(&self, token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret);
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
