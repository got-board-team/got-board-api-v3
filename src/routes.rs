pub mod matches {
    use crate::models::{Match, MatchAttr};
    use rocket_contrib::json::{Json, JsonValue};

    #[get("/")]
    pub fn all() -> JsonValue {
        json!(Match::all())
    }

    #[get("/<id>")]
    pub fn get(id: i32) -> JsonValue {
        json!(Match::get(id))
    }

    #[post("/", format = "json", data = "<mat>")]
    pub fn create(mat: Json<MatchAttr>) -> JsonValue {
        let match_attributes = MatchAttr { ..mat.into_inner() };
        json!(Match::create(match_attributes))
    }

    #[put("/<id>", format = "json", data = "<mat>")]
    pub fn update(id: i32, mat: Json<MatchAttr>) -> JsonValue {
        let match_attributes = MatchAttr { ..mat.into_inner() };
        json!(Match::update(id, match_attributes))
    }

    #[delete("/<id>")]
    pub fn delete(id: i32) -> JsonValue {
        json!({ "success": Match::delete(id) })
    }

    #[post("/<id>/join", format = "json", data = "<user_id>")]
    pub fn join(id: i32, user_id: Json<i32>) -> JsonValue {
        json!(Match::join(id, user_id.0))
    }
}

pub mod users {
    use crate::models::User;
    use rocket_contrib::json::JsonValue;

    #[get("/")]
    pub fn all() -> JsonValue {
        json!(User::all())
    }
}

pub mod websocket {
    use crate::models::Message;
    use dotenv::dotenv;
    use pusher::Pusher;
    use rocket_contrib::json::{Json, JsonValue};

    #[post("/messages", format = "json", data = "<message>")]
    pub fn pusher_message(message: Json<Message>) -> JsonValue {
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
}
