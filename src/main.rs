fn main() {
    let mut s = String::new(); // Create a new empty String
    

    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string(); 
    // Using the to_string method to create a String from a string literal

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s); // s: foobar
}