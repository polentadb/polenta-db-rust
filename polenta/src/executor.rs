use crate::storage;

pub(crate) fn run(statement: &str) -> (bool, String) {
    println!("{}", statement);
    (true, String::from("executed -> ") + statement)
}

fn create_user(user_name: &str) {
    storage::create_user(user_name);
}