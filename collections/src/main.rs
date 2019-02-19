mod vectors;

fn main() {
    // In real world scenarios, Rust will infer the type from the elements inserted into the vector,
    // We should initialize the vector with some initial elements into it, we can also use vec! macro
    // for convenience
    let mut m = vec![1, 2, 3, 4, 5, 6, 7];
    m.push(1);

    let mut n = Vec::new();
    n.push(5);
    n.push(4);
    n.push(6);
    n.push(7);
    n.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    match v.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element"),
    }
}
