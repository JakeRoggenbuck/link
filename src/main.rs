#![feature(decl_macro)]
use rocket::{catchers, routes};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

#[database("redirect-api")]
pub struct DbConn(diesel::PgConnection);

pub mod communicator;
pub mod models;
pub mod routes;
pub mod schema;
pub mod forms;

use self::routes::*;

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .attach(DbConn::fairing())
        .mount("/", routes![root, get_redirect])
        .mount(
            "/api/",
            routes![new_redirect, get_redirects, new_token, get_version, hashit],
        )
        .launch();
}
