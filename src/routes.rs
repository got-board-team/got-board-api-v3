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
