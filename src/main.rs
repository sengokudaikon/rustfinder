#![feature(proc_macro_hygiene, decl_macro)]
mod usecases;
mod application;
mod domain;
mod infrastructure;
mod persistence;
mod port;
mod presentation;

use crate::presentation::auth::AuthController;

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
        .mount("/", routes![index])
        .manage(controller)
        .mount("/auth", routes![presentation::auth::login, presentation::auth::sign_up])
}
