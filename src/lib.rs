#![feature(decl_macro)]
#[macro_use] extern crate rocket;

pub use rocket::routes;
use rocket::response::{Responder, NamedFile};

#[get("/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello, {name}!")
}

#[get("/")]
pub fn index() -> Result<impl Responder<'static>, failure::Error> {
    NamedFile::open("./pages/index.html").map_err(|e| e.into())
}

#[get("/style")]
pub fn style() -> Result<impl Responder<'static>, failure::Error> {
    NamedFile::open("./styles/style.css").map_err(|e| e.into())
}