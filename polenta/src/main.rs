mod executor;
mod storage;
fn main() {
    println!("Starting Polenta DB");

    let mut statements: Vec::<String> = Vec::new();
    statements.push(String::from("create bag person (id sequence, name string, age int)"));
    statements.push(String::from("create bag animals (name string)"));
    statements.push(String::from("create table animals (id sequence, name string)"));
    statements.push(String::from("create table animals (name string)"));
    statements.push(String::from("create function add (a int, b int) int"));
    statements.push(String::from("create user developer"));
    statements.push(String::from("create bag person (id sequence, name string, age int)"));
    statements.push(String::from("create user dba"));
    statements.push(String::from("create user admin"));
    statements.push(String::from("insert into person (name, age) values (\"John\", 30)"));
    statements.push(String::from("insert into person (\"Mary\", 40)"));
    statements.push(String::from("insert into employee (name, age) values (\"John\", 30)"));
    statements.push(String::from("select * from person where age = 20"));
    statements.push(String::from("select * from person where age = 30"));
    statements.push(String::from("select * from person where age = 40"));
    statements.push(String::from("select * from role"));

    for statement in statements {
        let (_, message) = executor::run(statement.as_str());
        println!("{}", message);
    }
}