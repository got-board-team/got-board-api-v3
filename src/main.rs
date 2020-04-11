#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_cors;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate pusher;
extern crate rustc_serialize;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

fn main() {
    let default_cors = rocket_cors::CorsOptions::default()
        .to_cors()
        .expect("error while building CORS");

    rocket::ignite()
        .attach(default_cors)
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
        .mount(
            "/users",
            routes![
                routes::users::all,
                routes::users::get,
                routes::users::create,
                routes::users::update,
                routes::users::delete,
            ],
        )
        .mount("/", routes![routes::websocket::pusher_message])
        .launch();
}
