#![feature(decl_macro)]
use rocket::request::{Form, FromForm};
use rocket::Request;
use rocket::*;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

fn main() {
    println!("Hello, world!");
}
