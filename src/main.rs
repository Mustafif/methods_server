use std::borrow::BorrowMut;

use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::Request;
use rocket::State;
use rocket::{get, post, delete, put, launch, routes, catch, catchers};
use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static!{
    static ref data: Mutex<Data> = Mutex::new(Data { value: "Default".to_owned() });
}

// Store data for GET/PUT requests
#[derive(Debug, Clone)]
struct Data {
    value: String,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the simple web server!"
}

#[get("/data")]
fn get_data() -> String {
    let d = data.lock().unwrap();
    d.value.clone()
}

#[post("/pos_data", data = "<new_value>")]
fn post_data(new_value: String) -> String {
    let mut d = data.lock().unwrap();
    d.value = new_value.clone();
    format!("Data updated to {}", new_value)
}

#[put("/put_data", data = "<new_value>")]
fn put_data(new_value: String) -> String {
    let mut d = data.lock().unwrap();
    d.value = new_value.clone();
    format!("Data updated to {}", new_value)
}

#[delete("/del_data")]
fn delete_data() -> String {
    let mut d = data.lock().unwrap();
    d.value = "".to_string();
    "Data deleted".to_string()
}

#[catch(404)]
fn not_found() -> &'static str {
    "Not found"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_data, post_data, put_data, delete_data])
        .manage(Data { value: "".to_string() })
        .register("/", catchers![not_found])
}
