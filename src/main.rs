#[macro_use]
extern crate rocket;

use std::env;

use log::info;
use rocket::{http::Status, serde::json::Json};
use serde_json::{json, Value};
use structs::GameState;

mod pathfinding;
mod structs;

#[get("/")]
fn handle_index() -> Json<Value> {
    Json(json!({
        "apiversion": "1",
        "author": "niko",
        "color": "#8155a3",
        "head": "all-seeing",
        "tail": "default",
    }))
}

#[post("/start", format = "json")]
fn start_game() -> Status {
    info!("Starting new game");
    Status::Ok
}

#[post("/start", format = "json")]
fn end_game() -> Status {
    info!("Game complete");
    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>) -> Json<Value> {
    let response = pathfinding::process_move(&move_req.board, &move_req.you);

    Json(json!(response))
}

#[launch]
fn rocket() -> _ {
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", port);
    }
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Starting Battlesnake Server...");
    rocket::build().mount(
        "/",
        routes![handle_index, start_game, end_game, handle_move],
    )
}
