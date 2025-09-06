

fn main() {
    let data = vec![5,1,2,2,5,4,3,1,4,5,5];
    
    let sum: i32 = data.iter().sum();
    
    println!("Mean: {}", sum as f64 / data.len() as f64);
}

