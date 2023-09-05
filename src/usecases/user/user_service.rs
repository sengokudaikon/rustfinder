use rocket::response::content::Json;

pub async fn signup(signup_request: Json<SignupRequest>) -> Result<Json<SignupResponse>, Status> {
    let user_repository = UserRepository::new(); // Replace with your UserRepository implementation
    let sign_up_response = register_user(sign_up_request.into_inner(), &user_repository).await?;
    Ok(Json(sign_up_response))
    Ok(Json(signup_response))
}