use diesel::prelude::*;

use super::model::NewUsers;

pub fn create_users(
    conn: &mut MysqlConnection,
    new_users: NewUsers,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::users;

    let row_insert = diesel::insert_into(users::table)
        .values(&new_users)
        .execute(conn);

    return row_insert;
}
