fn another_function() {
    println!("Another function");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");
    another_function();

    let x = {
      let y = 5;
        y + 1
    };
    let m = plus_one(x);
    println!("value is: {}", m);
}
