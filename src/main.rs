fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // ! This will cause a panic
    // let does_not_exist = &v[100];
    // println!("does_not_exist: {:?}", does_not_exist);
    // let does_not_exist = v.get(100);
    // println!("does_not_exist: {:?}", does_not_exist);

    // ! Iterating over the values in a vector
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}