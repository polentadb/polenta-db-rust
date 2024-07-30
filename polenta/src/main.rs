fn main() {
    let _age: i32 = 5;
    let _size: u32 = 5;

    let _salary = 2_000.0; //f64

    let _price: f32 = 3.0;

    let _employee = false;

    let _sold: bool = true;

    let _letter = 'z';

    let person = ('z', 40, 3000.0, true);

    println!("{}", person.0);

    let _person_name = person.0;
    let _person_age = person.1;

    let (_p_name, _p_age, _p_salary, _p_manager) = person;

    let _arr1 = [1, 2, 3, 4, 5];

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr2[0]);


    let mut vec1 = vec![1, 2, 3, 4, 5];
    vec1.push(6);
    println!("{}", vec1[5]);

    let mut vec2: Vec<i32> = vec![];
    vec2.push(10);

    let _vec3: Vec::<i32> = Vec::new();

    println!("Starting Polenta DB");
}
