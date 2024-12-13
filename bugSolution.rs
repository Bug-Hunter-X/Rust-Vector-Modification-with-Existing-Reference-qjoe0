fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Correct approach: Copy the value before modifying the vector
    let x = vec[0];
    vec.push(4);

    println!("x = {}", x);
}