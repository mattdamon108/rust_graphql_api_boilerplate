extern crate rust_graphql_api_boilerplate;

use self::db::*;
use self::user::*;
use rust_graphql_api_boilerplate::*;

fn main() {
    let connection = establish_connection();

    // delete_user(&1, &connection);
}