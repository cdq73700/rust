use diesel::prelude::*;

use super::model::NewUsers;

pub fn create_users(conn: &mut MysqlConnection, new_users: NewUsers) {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&new_users)
        .execute(conn)
        .expect("Error saving new users");
}
