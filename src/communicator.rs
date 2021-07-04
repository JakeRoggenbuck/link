use super::schema;
use super::schema::{redirects, tokens};
use super::models::*;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use sha2::{Digest, Sha256};
use super::DbConn;
use std::env;

/// Make a sha256 sum of a token
pub fn hash(token: String) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}

/// Check if the key passed in is either in the .env as AUTH_KEY or if it's in the database table
/// called tokens, this means the user has authority to make redirects, and add tokens
pub fn is_valid(key: &str, conn: DbConn) -> bool {
    let key: String = hash(key.to_string());

    // First check if the auth key is in the .env
    match env::var("AUTH_KEY") {
        Ok(t) => {
            if key == t {
                return true;
            }
        }
        Err(_) => (),
    }

    use self::schema::tokens::dsl::*;
    // Check if the key is in the database
    let result: Vec<Token> = tokens
        .filter(token.eq(key))
        .limit(1)
        .load::<Token>(&*conn)
        .expect("Error loading a token");

    result.len() == 1
}

/// Write a single redirect to the database
pub fn write_redirect(new: NewRedirect, conn: &diesel::PgConnection) {
    diesel::insert_into(redirects::table)
        .values(&new)
        .execute(conn)
        .expect("Error saving new redirect");
}

pub fn increment_count(given_alias: &str, conn: &diesel::PgConnection) {
    use self::schema::redirects::dsl::*;
    let result = diesel::update(redirects)
        .filter(alias.eq(given_alias))
        .set(count.eq(count + 1))
        .execute(conn);
    match result {
        Ok(_) => (),
        Err(e) => eprintln!("Could not update count for {}\n {}", given_alias, e),
    }
}

/// Write a single token to the database
pub fn write_token(new: NewToken, conn: &diesel::PgConnection) {
    diesel::insert_into(tokens::table)
        .values(&new)
        .execute(conn)
        .expect("Error saving new token");
}

/// When you pass in an expected alias, check if it's in the database, and return the UrlRedirect
/// if it's there
pub fn get_redirect_by_alias(
    expected_alias: &str,
    conn: &diesel::PgConnection,
) -> Vec<UrlRedirect> {
    use self::schema::redirects::dsl::*;
    let result: Vec<UrlRedirect> = redirects
        .filter(alias.eq(expected_alias))
        .limit(1)
        .load::<UrlRedirect>(conn)
        .expect("Error loading an alias");
    return result;
}

/// Return a large amount of redirects for debugging and managing usage
pub fn get_many_redirects(conn: &diesel::PgConnection) -> Vec<UrlRedirect> {
    use self::schema::redirects::dsl::*;
    let result: Vec<UrlRedirect> = redirects
        .limit(100)
        .load::<UrlRedirect>(conn)
        .expect("Error loading some redirects");
    return result;
}
