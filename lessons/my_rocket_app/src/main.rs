#[macro_use] extern crate rocket;

use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::State; 
use rocket::http::Status;
use rocket::response::status;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
    #[serde(default = "default_true")]
    active: bool
}

#[derive(Deserialize)]
struct UpdateUser {
    id: u32,
    name: Option<String>,
    active: Option<bool>,
}

fn default_true() -> bool {
    true
}

#[get("/")]
fn index() -> &'static str {
    "Hello, Users API!"
}

// Create

#[post("/api/users", data = "<new_user_data>")]
fn create_user(new_user_data: Json<NewUser>, user_list: &State<Mutex<Vec<User>>>) -> Json<User> {
    let mut users = user_list.lock().unwrap();
    // Find the next available ID
    let new_id = users.iter().map(|user| user.id).max().unwrap_or(0) + 1 as u32;
  
    let new_user = User {
        id: new_id,
        name: new_user_data.name.clone(),
        active: new_user_data.active,
    };
    users.push(new_user.clone());
    Json(new_user.clone())
}

// Read

#[get("/api/users")]
fn get_users(user_list: &State<Mutex<Vec<User>>>) -> Json<Vec<User>> {
    let users = user_list.lock().unwrap();
    Json(users.clone())
}

#[get("/api/users/<id>")]
fn get_user_by_id(id: u32, user_list: &State<Mutex<Vec<User>>>) -> Option<Json<User>> {
    let users = user_list.lock().unwrap();
    for user in users.iter() {
        if user.id == id {
            return Some(Json(user.clone()));
        }
    }
    None
}

// Update

#[put("/api/users", data = "<update_user_data>")]
fn update_user(update_user_data: Json<UpdateUser>, user_list: &State<Mutex<Vec<User>>>) -> Option<Json<User>> {
    let mut users = user_list.lock().unwrap();
    for user in users.iter_mut() {
        if user.id == update_user_data.id {
            if let Some(name) = update_user_data.name.clone() {
                user.name = name;
            }
            if let Some(active) = update_user_data.active {
                user.active = active;
            }
            return Some(Json(user.clone()));
        }
    }
    None
}

// Delete

#[delete("/api/users/<id>")]
fn delete_user(id: u32, user_list: &State<Mutex<Vec<User>>>) -> Result<status::NoContent, Status> {
    let mut users = user_list.lock().unwrap();
    for i in 0..users.len() {
        if users[i].id == id {
            users.remove(i);
            return Ok(status::NoContent);
        }
    }
    Err(Status::NotFound)
}


#[launch]
fn rocket() -> _ {
    let user_list: Mutex<Vec<User>> = Mutex::new(vec![
        User { id: 1, name: "Roo".to_string(), active: true },
        User { id: 2, name: "メガネ君".to_string(), active: true },
    ]);
    
    rocket::build().mount("/", routes![index, create_user, get_users, get_user_by_id, update_user, delete_user]).manage(user_list)
}