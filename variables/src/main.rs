
fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");

    const NUM: u32 = 10 * 10;

    println!("num is: {}", num);

    let tup = ("test", 5, 1.2, 1.22);
    let months: [&str; 5] = ["test", "testing", "testing", "testing", "test"];
}
