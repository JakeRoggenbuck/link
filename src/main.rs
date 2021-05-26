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

fn write_redirect(new: NewRedirect, conn: &diesel::PgConnection) {
    diesel::insert_into(redirects::table)
        .values(&new)
        .execute(conn)
        .expect("Error saving new post");
}

fn get_redirect_by_alias(expected_alias: &str, conn: &diesel::PgConnection) -> Vec<Redirect> {
    use self::schema::redirects::dsl::*;
    let result = redirects
        .filter(alias.eq(expected_alias))
        .limit(1)
        .load::<Redirect>(conn)
        .expect("Error loading posts");
    return result;
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/")]
fn root() -> String {
    format!("Hello world!")
}

#[get("/redirect/<alias>")]
fn get_redirect(alias: String, conn: DbConn) -> String {
    let redirect = get_redirect_by_alias(alias.as_str(), &conn);
    format!("{:?}", redirect)
}

#[post("/newredirect", data = "<redirect_form>")]
fn new_redirect(redirect_form: Form<RedirectForm>, conn: DbConn) -> String {
    let redirect: RedirectForm = redirect_form.into_inner();
    let new = models::NewRedirect {
        alias: redirect.alias.as_str(),
        url: redirect.url.as_str(),
    };

    if get_redirect_by_alias(new.alias, &conn).len() == 0 {
        write_redirect(new.clone(), &conn);
        return format!("Wrote to database {} -> {}", new.alias, new.url);
    } else {
        return format!("Alias {} already exists", new.alias);
    }
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .attach(DbConn::fairing())
        .mount("/", routes![root])
        .mount("/api/", routes![new_redirect, get_redirect])
        .launch();
}
