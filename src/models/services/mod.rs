pub mod matches {
    use crate::db;
    use crate::schema::{pieces};
    use diesel::prelude::*;

    pub fn setup(match_id: i32, players_count: i32) {
        match players_count {
            3 => setup_3_players(match_id),
            4 => setup_4_players(match_id),
            5 => setup_5_players(match_id),
            6 => setup_6_players(match_id),
            _ => println!("Nothing")
        }

        create_piece(match_id, String::from("wildings_token"), 382, 81, String::from("map"), None);
    }

    fn create_piece(match_id: i32, piece_type: String, x: i32, y: i32, location: String, house_name: Option<String>) {
        let connection = db::establish_connection();
        diesel::insert_into(pieces::table)
            .values((
                pieces::columns::match_id.eq(&match_id),
                pieces::columns::piece_type.eq(&piece_type),
                pieces::columns::x.eq(&x),
                pieces::columns::y.eq(&y),
                pieces::columns::location.eq(&location),
                pieces::columns::house_name.eq(&house_name),
            ))
            .execute(&connection)
            .expect("Could not create piece");
    }

    fn setup_3_players(match_id: i32) {
        create_piece(match_id, String::from("ship"), 1262, 1985, String::from("map"), Some(String::from("baratheon")));
        create_piece(match_id, String::from("ship"), 1372, 2138, String::from("map"), Some(String::from("baratheon")));
        create_piece(match_id, String::from("knight"), 1392, 1797, String::from("map"), Some(String::from("baratheon")));
        create_piece(match_id, String::from("footman"), 1295, 1804, String::from("map"), Some(String::from("baratheon")));
        create_piece(match_id, String::from("footman"), 1098, 2000, String::from("map"), Some(String::from("baratheon")));

        create_piece(match_id, String::from("ship"), 1208, 738, String::from("map"), Some(String::from("stark")));
        create_piece(match_id, String::from("knight"), 625, 851, String::from("map"), Some(String::from("stark")));
        create_piece(match_id, String::from("footman"), 773, 544, String::from("map"), Some(String::from("stark")));
        create_piece(match_id, String::from("footman"), 852, 810, String::from("map"), Some(String::from("stark")));

        create_piece(match_id, String::from("knight"), 328, 1826, String::from("map"), Some(String::from("lannister")));
        create_piece(match_id, String::from("footman"), 348, 1690, String::from("map"), Some(String::from("lannister")));
        create_piece(match_id, String::from("ship"), 133, 1759, String::from("map"), Some(String::from("lannister")));
        create_piece(match_id, String::from("footman"), 476, 1820, String::from("map"), Some(String::from("lannister")));
    }

    fn setup_4_players(match_id: i32) {
        setup_3_players(match_id);

        create_piece(match_id, String::from("ship"), 1121, 2475, String::from("map"), Some(String::from("martell")));
        create_piece(match_id, String::from("knight"), 1010, 2600, String::from("map"), Some(String::from("martell")));
        create_piece(match_id, String::from("footman"), 1107, 2611, String::from("map"), Some(String::from("martell")));
        create_piece(match_id, String::from("footman"), 814, 2698, String::from("map"), Some(String::from("martell")));
    }

    fn setup_5_players(match_id: i32) {
        setup_4_players(match_id);

        create_piece(match_id, String::from("ship"), 87, 1557, String::from("map"), Some(String::from("greyjoy")));
        create_piece(match_id, String::from("ship"), 325, 1350, String::from("map"), Some(String::from("greyjoy")));
        create_piece(match_id, String::from("knight"), 198, 1472, String::from("map"), Some(String::from("greyjoy")));
        create_piece(match_id, String::from("footman"), 281, 1479, String::from("map"), Some(String::from("greyjoy")));
        create_piece(match_id, String::from("footman"), 458, 1164, String::from("map"), Some(String::from("greyjoy")));
    }

    fn setup_6_players(match_id: i32) {
        setup_5_players(match_id);

        create_piece(match_id, String::from("ship"), 77, 2504, String::from("map"), Some(String::from("tyrell")));
        create_piece(match_id, String::from("knight"), 389, 2171, String::from("map"), Some(String::from("tyrell")));
        create_piece(match_id, String::from("footman"), 406, 2262, String::from("map"), Some(String::from("tyrell")));
        create_piece(match_id, String::from("footman"), 498, 2308, String::from("map"), Some(String::from("tyrell")));
    }
}
