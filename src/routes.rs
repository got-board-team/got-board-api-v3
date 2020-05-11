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

    #[get("/<id>/pieces")]
    pub fn match_pieces(id: i32) -> JsonValue {
        json!(Piece::all(id))
    }

    #[post("/<id>/pieces", format = "json", data = "<piece_params>")]
    pub fn create_piece(id: i32, piece_params: Json<PieceParams>) -> JsonValue {
        let new_piece = PieceParams {
            ..piece_params.into_inner()
        };
        json!(Piece::create(id, new_piece))
    }

    #[put("/<_id>/pieces/<piece_id>", format = "json", data = "<piece>")]
    pub fn update_piece(_id: i32, piece_id: i32, piece: Json<PieceParams>) -> JsonValue {
        let updated_piece = PieceParams {
            ..piece.into_inner()
        };
        json!(Piece::update(piece_id, updated_piece))
    }

    #[delete("/<_id>/pieces/<piece_id>")]
    pub fn delete_piece(_id: i32, piece_id: i32) -> JsonValue {
        json!({ "success": Piece::delete(piece_id) })
    }
}

pub mod users {
    use crate::models::{SearchableUserAttr, User, UserAttr};
    use rocket::request::Form;
    use rocket_contrib::json::{Json, JsonValue};

    #[get("/")]
    pub fn all() -> JsonValue {
        json!(User::all())
    }

    #[get("/<id>")]
    pub fn get(id: i32) -> JsonValue {
        json!(User::get(id))
    }

    // TODO: Implement 404
    #[get("/filter?<user..>")]
    pub fn filter(user: Form<SearchableUserAttr>) -> JsonValue {
        json!(User::filter(user))
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
    use pusher::PusherBuilder;
    use rocket_contrib::json::{Json, JsonValue};
    use tokio::runtime::Runtime;

    #[post("/messages", format = "json", data = "<message>")]
    pub fn pusher_message(message: Json<Message>) -> JsonValue {
        dotenv().ok();
        let api_id = dotenv::var("PUSHER_API_ID").expect("API_ID is not loaded");
        let key = dotenv::var("PUSHER_KEY").expect("Pusher KEY not set");
        let app_secret = dotenv::var("PUSHER_APP_SECRET").expect("Pusher APP_SECRET not set");
        let mut pusher = PusherBuilder::new(&api_id, &key, &app_secret).finalize();
        let msg = Message {
            ..message.into_inner()
        };
        let push_payload = Runtime::new()
            .expect("Failed to create Tokio runtime")
            .block_on(pusher.trigger("game", "update", &msg));

        match push_payload {
            Ok(_) => json!({ "success": &msg }),
            Err(error) => json!({ "error": error }),
        }
    }
}
