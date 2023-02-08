use diesel::prelude::*;

use super::model::{
    Users,
    NewUsers,
};

pub fn create_users(conn: &mut MysqlConnection, id: &str, name: &str) -> Users {

    use crate::schema::users;

    let new_users = NewUsers { id, name };

    diesel::insert_into(users::table)
        .values(&new_users)
        .execute(conn)
        .expect("Error saving new users");

    users::table.order(users::id.desc()).first(conn).unwrap()
}