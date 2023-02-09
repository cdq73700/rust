use diesel::prelude::*;

#[derive(Queryable)]
pub struct Users {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUsers<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
