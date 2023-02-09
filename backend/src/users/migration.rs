use super::model::NewUsers;
use backend::establish_connection;
use diesel::prelude::*;
use pwhash::bcrypt;

use super::create::create_users;

pub fn migration_users() {
    let connection: &mut MysqlConnection = &mut establish_connection();

    let new_users: NewUsers = NewUsers {
        email: "test@test.com".to_string(),
        name: "test".to_string(),
        password: bcrypt::hash("test").unwrap(),
        created_by: "test".to_string(),
        updated_by: "test".to_string(),
    };

    create_users(connection, new_users);
}
