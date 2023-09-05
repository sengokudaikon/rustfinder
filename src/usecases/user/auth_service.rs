use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::http::Status;
use crate::application::auth::login_cqrs::LoginRequest;
use crate::domain::user::{User, Claims};
use crate::infrastructure::user_repository::UserRepository;

pub async fn authenticate_user(login_request: LoginRequest, user_repository: &UserRepository) -> Result<LoginResponse, Status> {
    // Authenticate the user using the UserRepository
    // If the user is authenticated, generate a JWT using the encode_jwt function
}

pub async fn encode_jwt(user: &User, secret: &[u8], expiration: usize) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: user.id,
        exp: expiration,
        roles: user.roles.clone(),
    };
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret);
    encode(&header, &claims, &encoding_key)
}

pub async  fn decode_jwt(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret);
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}