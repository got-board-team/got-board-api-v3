#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate pusher;
extern crate rustc_serialize;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;
use models::Message;
use pusher::Pusher;
use rocket_contrib::json::{Json, JsonValue};

#[post("/messages", format = "json", data = "<message>")]
fn pusher_message(message: Json<Message>) -> JsonValue {
    dotenv().ok();
    let api_id = dotenv::var("PUSHER_API_ID").expect("API_ID is not loaded");
    let key = dotenv::var("PUSHER_KEY").expect("Pusher KEY not set");
    let app_secret = dotenv::var("PUSHER_APP_SECRET").expect("Pusher APP_SECRET not set");
    let mut pusher = Pusher::new(&api_id, &key, &app_secret).finalize();
    let msg = Message {
        ..message.into_inner()
    };
    match pusher.trigger("game", "update", &msg) {
        Ok(_) => json!({ "success": &msg }),
        Err(error) => json!({ "error": error }),
    }
}

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
        .mount("/", routes![pusher_message])
        .launch();
}
