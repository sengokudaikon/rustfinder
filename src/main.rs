#![feature(proc_macro_hygiene, decl_macro)]
mod usecases;
mod application;
mod domain;
mod infrastructure;
mod persistence;
mod port;
mod presentation;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world, from the rocket-base template! :)"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
