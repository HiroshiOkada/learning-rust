#[macro_use] extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, search])
}