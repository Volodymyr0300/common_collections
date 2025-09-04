fn main() {
    let v1: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v1);
    
    let v2 = vec![1, 2, 3];
    println!("Empty vector: {:?}", v2);

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("Vector after pushes: {:?}", v3);

    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here   

    
}