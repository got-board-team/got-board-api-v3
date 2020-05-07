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
extern crate futures; // `pusher` is async, so we need to block on the future in this example
extern crate pusher;
extern crate rustc_serialize;
extern crate serde_json;

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
                routes::matches::match_pieces,
                routes::matches::create_piece,
                routes::matches::update_piece,
                routes::matches::delete_piece,
            ],
        )
        .mount(
            "/users",
            routes![
                routes::users::all,
                routes::users::get,
                routes::users::filter,
                routes::users::create,
                routes::users::update,
                routes::users::delete,
            ],
        )
        .mount("/", routes![routes::websocket::pusher_message])
        .launch();
}
