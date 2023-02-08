mod users;
mod schema;

use users::migration::{
    migration_users,
};

fn main() {
    migration_users();
}


