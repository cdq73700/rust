use backend::establish_connection;
use diesel::prelude::*;

use super::create::create_users;

pub fn migration_users() {
    let connection: &mut MysqlConnection = &mut establish_connection();

    let id: &str = "123456789";
    let name: &str = "test";

    let _users: super::model::Users = create_users(connection, id, name);
}
