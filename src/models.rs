use super::schema::redirects;

#[derive(Queryable, Clone, Debug)]
pub struct Redirect {
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
