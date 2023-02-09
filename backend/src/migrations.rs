mod schema;
mod users;

use env_logger;
use std::env;

use users::migration::migration_users;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    migration_users();
}
