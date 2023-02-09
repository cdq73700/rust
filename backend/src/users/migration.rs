use backend::establish_connection;
use diesel::prelude::*;
use uuid::Uuid;

use super::create::create_users;

pub fn migration_users() {
    let connection: &mut MysqlConnection = &mut establish_connection();

    let id: &str = &Uuid::new_v4().to_string();
    let name: &str = "test";

    let _users: super::model::Users = create_users(connection, id, name);
}
