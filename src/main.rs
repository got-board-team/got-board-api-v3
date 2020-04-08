#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate pusher;
extern crate rustc_serialize;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

fn main() {
    rocket::ignite()
        .mount(
            "/matches",
            routes![
                routes::matches::all,
                routes::matches::get,
                routes::matches::create,
                routes::matches::update,
                routes::matches::delete,
                routes::matches::join,
            ],
        )
        .mount("/users", routes![routes::users::all])
        .mount("/", routes![routes::websocket::pusher_message])
        .launch();
}
