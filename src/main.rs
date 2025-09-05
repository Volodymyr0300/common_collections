fn main() {
    /* String Concatenation 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    */


    /*
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
     */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // s1, s2, and s3 are still valid here because format! does not take ownership
   println!("Formatted string: {}", s);
   println!("Original strings: {}, {}, {}", s1, s2, s3);
   
    for c in "Hello".chars() {
        println!("{}", c);
    }
    for b in "Hello".bytes() {
        println!("{}", b);
    }

}