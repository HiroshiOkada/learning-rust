#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

#[derive(Deserialize)]
struct NewUser {
    name: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/search?<term>")]
fn search(term: Option<String>) -> String {
    match term {
        Some(t) => format!("Searching for: {}", t),
        None => "Please provide a search term.".to_string(),
    }
}

#[get("/api/user")]
fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        name: "Roo".to_string(),
        active: true,
    };
    Json(user)
}

#[post("/api/users", data = "<user_data>")]
fn create_user(user_data: Json<NewUser>) -> String {

    format!("User '{}' created successfully!", user_data.name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, search, get_user, create_user])
}