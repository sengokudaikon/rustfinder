#![feature(proc_macro_hygiene, decl_macro)]
mod usecases;
mod application;
mod domain;
mod infrastructure;
mod persistence;
mod port;
mod presentation;

use utoipa_swagger_ui::SwaggerUi;
use crate::presentation::auth::AuthController;
use crate::presentation::apidoc::ApiDoc;
use utoipa::OpenApi;
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world, from the rocket-base template! :)"
}

#[launch]
async fn rocket() -> _ {
    let controller = AuthController::new().await;
    rocket::build()
        .mount("/", SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .manage(controller)
        .mount("/api", routes![presentation::auth::login, presentation::auth::register])
}
