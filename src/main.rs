#![feature(decl_macro)]
use rocket::request::{Form, FromForm};
use rocket::Request;
use rocket::*;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use std::iter::Iterator;

#[database("redirect-api")]
struct DbConn(diesel::PgConnection);

pub mod models;
pub mod schema;

use models::{NewRedirect, Redirect};
use schema::redirects;

#[derive(FromForm, Debug)]
struct RedirectForm {
    alias: String,
    url: String,
}

fn main() {
    println!("Hello, world!");
}
