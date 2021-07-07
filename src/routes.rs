use super::communicator::*;
use super::forms::{RedirectForm, TokenForm};
use super::models::{NewRedirect, NewToken, UrlRedirect};
use super::DbConn;
use rocket::http::Status;
use rocket::request::{Form, FromRequest};
use rocket::response::Redirect;
use rocket::Request;
use rocket::*;

#[derive(Debug)]
pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Get the headers from the request, and check for the api key
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            // Give the key and the database connection to check if the key is in the database
            1 if is_valid(keys[0], request.guard::<DbConn>().unwrap()) => {
                Outcome::Success(ApiKey(keys[0].to_string()))
            }
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/")]
pub fn root() ->  Redirect {
    Redirect::to(format!("https://github.com/JakeRoggenbuck/link"))
}

#[get("/hash/<thing>")]
pub fn hashit(thing: String) -> String {
    hash(thing)
}

#[get("/<alias>")]
pub fn get_redirect(alias: String, conn: DbConn) -> Redirect {
    let redirect: Vec<UrlRedirect> = get_redirect_by_alias(alias.as_str(), &conn);
    // If there is a single redirect, meaning not a duplicate, and one exists
    if redirect.len() == 1 {
        // Extract the url from the redirect
        let url: String = redirect.get(0).unwrap().url.to_string();
        let alias: String = redirect.get(0).unwrap().alias.to_string();

        increment_count(&alias, &conn);
        if url != "" {
            // Sends the user to this url
            return Redirect::to(format!("{}", url));
        }
    }
    // Sends the user to the github pages 404
    return Redirect::to(format!("https://jr0.org/404.html"));
}

#[get("/redirects")]
pub fn get_redirects(_key: ApiKey, conn: DbConn) -> String {
    let redirects: Vec<UrlRedirect> = get_many_redirects(&conn);
    // Check if there are redirects
    if redirects.len() != 0 {
        // Format the redirect with an arrow and tabs
        let redirect_strings: String = redirects
            .iter()
            .map(|x| {
                format!(
                    "{}: {} -> {}\n",
                    x.count.to_owned(),
                    x.alias.to_owned(),
                    x.url.as_str()
                )
            })
            .collect();
        format!("{}", redirect_strings)
    } else {
        format!("No redirects to display")
    }
}

#[get("/version")]
pub fn get_version(_key: ApiKey) -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[post("/newredirect", data = "<redirect_form>")]
pub fn new_redirect(_key: ApiKey, redirect_form: Form<RedirectForm>, conn: DbConn) -> String {
    let redirect: RedirectForm = redirect_form.into_inner();
    // Takes the fields from the form, and turn it into a struct that derives Insertable
    let new: NewRedirect = NewRedirect {
        alias: redirect.alias.as_str(),
        url: redirect.url.as_str(),
        count: &0,
    };

    // Checks if the alias exists in the database, as to not overwrite it
    if get_redirect_by_alias(new.alias, &conn).len() == 0 {
        write_redirect(new.clone(), &conn);
        format!("Wrote to database {} -> {}", new.alias, new.url)
    } else {
        format!("Alias {} already exists", new.alias)
    }
}

#[post("/newtoken", data = "<token_form>")]
pub fn new_token(_key: ApiKey, token_form: Form<TokenForm>, conn: DbConn) -> String {
    let token_struct: TokenForm = token_form.into_inner();
    let hashed_token: String = hash(token_struct.token.clone());

    let new_token: NewToken = NewToken {
        token: hashed_token.as_str(),
    };

    write_token(new_token, &conn);
    format!("Wrote new token")
}
