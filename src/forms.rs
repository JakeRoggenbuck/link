use rocket::request::FromForm;

/// The form given the alias and the url from the request
#[derive(FromForm, Debug)]
pub struct RedirectForm {
    pub alias: String,
    pub url: String,
}

/// The form given the token from the request
#[derive(FromForm, Debug)]
pub struct TokenForm {
    pub token: String,
}
