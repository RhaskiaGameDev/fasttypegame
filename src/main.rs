mod checkanswer;
mod csv;
use crate::csv::*;


#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::json::json;
use rocket::serde::json::Value;
use rocket::fs;
use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Message<'r> {
    contents: &'r str,
}

#[derive(Serialize, Deserialize)]
struct CurrLength (usize);

#[post("/message", data = "<json>")]
fn update(json: Json<Message>) -> String  {
    let answers = &return_answers("worldcities.csv");

    match in_answers(json.contents, &answers.0)
    {
        Some(a) => serde_json::to_string(&CurrLength( answers.1 )).unwrap(),
        None =>  serde_json::to_string(&CurrLength( 0 )).unwrap(),
    }
}

#[launch]
fn rocket() -> _ {
    println!("{:?}", clean_file("worldcities.csv", 0));

    rocket::build()
        .mount("/", routes![update])
        .mount("/", FileServer::from(relative!("static")))
}
