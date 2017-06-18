#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use rocket_contrib::{JSON};

mod db;

#[derive(Deserialize)]
struct Introduction {
    name: String
}

#[derive(Serialize)]
struct Greeting {
    greeting: String
}


#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

// end hello - now for some db action

#[derive(Deserialize)]
struct CreatingPost {
    title: String,
    body: String
}

#[derive(Serialize)]
struct Post {
    created: 
}

#[post("/hi", format = "application/json", data = "<introduction>")]
fn hi(introduction: JSON<Introduction>) -> JSON<Greeting> {
    JSON(Greeting {
        greeting: format!("Hello, {}!", introduction.name)
    })
}

#[post("/blog")]
fn new_post(db: DB, post: JSON<CreatingPost>) -> JSON<Post> {
    JSON
}

fn main() {
    rocket::ignite().mount("/", routes![hello, hi]).launch();
}
