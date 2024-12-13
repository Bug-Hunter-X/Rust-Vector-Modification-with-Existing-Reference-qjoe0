fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This is where the bug occurs.
    let x = &vec[0];
    vec.push(4); // This line will cause a runtime error

    println!("x = {}", x);
}