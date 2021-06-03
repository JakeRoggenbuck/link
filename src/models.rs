use super::schema::{redirects, tokens};
use serde::Deserialize;

#[derive(Queryable, Clone, Debug, Deserialize)]
pub struct UrlRedirect {
    pub id: i32,
    pub alias: String,
    pub url: String,
}

#[derive(Debug, Insertable, Clone)]
#[table_name = "redirects"]
pub struct NewRedirect<'a> {
    pub alias: &'a str,
    pub url: &'a str,
}

#[derive(Queryable, Clone, Debug, Deserialize)]
pub struct Token {
    pub id: i32,
    pub token: String,
}

#[derive(Debug, Insertable, Clone)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
    pub token: &'a str,
}
