use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Users {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUsers {
    pub email: String,
    pub name: String,
    pub password: String,
    pub created_by: String,
    pub updated_by: String,
}
