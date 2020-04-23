pub mod matches {
    use crate::models::{Match, MatchAttr, MatchesUsersAttr, Piece, PieceParams};
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

    #[post("/<id>/join", format = "json", data = "<join_params>")]
    pub fn join(id: i32, join_params: Json<MatchesUsersAttr>) -> JsonValue {
        let join_match_attributes = MatchesUsersAttr {
            ..join_params.into_inner()
        };
        json!(Match::join(
            id,
            join_match_attributes.user_id,
            join_match_attributes.house_name
        ))
    }

    #[post("/<id>/pieces", format = "json", data = "<piece_params>")]
    pub fn create_pieces(id: i32, piece_params: Json<PieceParams>) -> JsonValue {
        let new_piece = PieceParams {
            ..piece_params.into_inner()
        };
        json!(Piece::create(id, new_piece))
    }
}

pub mod users {
    use crate::models::{User, UserAttr};
    use rocket_contrib::json::{Json, JsonValue};

    #[get("/")]
    pub fn all() -> JsonValue {
        json!(User::all())
    }

    #[get("/<id>")]
    pub fn get(id: i32) -> JsonValue {
        json!(User::get(id))
    }

    #[post("/", format = "json", data = "<user>")]
    pub fn create(user: Json<UserAttr>) -> JsonValue {
        let user_attributes = UserAttr {
            ..user.into_inner()
        };
        json!(User::create(user_attributes))
    }

    #[put("/<id>", format = "json", data = "<user>")]
    pub fn update(id: i32, user: Json<UserAttr>) -> JsonValue {
        let user_attributes = UserAttr {
            ..user.into_inner()
        };
        json!(User::update(id, user_attributes))
    }

    #[delete("/<id>")]
    pub fn delete(id: i32) -> JsonValue {
        json!({ "success": User::delete(id) })
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
