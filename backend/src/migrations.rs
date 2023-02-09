mod schema;
mod users;

use users::migration::migration_users;

fn main() {
    migration_users();
}
