#[macro_use]
extern crate diesel;

use diesel::*;

table! {
    users {
        id -> Serial,
        name -> VarChar,
    }
}

fn main() {
    use self::users::dsl::*;

    let connection = Connection::establish("").unwrap();
    let select_id = users.select(id);
    let select_name = users.select(name);

    let ids: Vec<i32> = connection.query_all(select_name).unwrap().collect();
    //~^ ERROR the trait `diesel::query_source::Queriable<diesel::types::VarChar>` is not implemented for the type `i32`
    //~| ERROR E0277
    let names: Vec<String> = connection.query_all(select_id).unwrap().collect();
    //~^ ERROR the trait `diesel::query_source::Queriable<diesel::types::Integer>` is not implemented
    //~| ERROR E0277
}
