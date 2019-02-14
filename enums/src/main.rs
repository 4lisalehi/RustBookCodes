enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("Quarter of state: {:?} ", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_underline(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (),
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let cent = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("which is {} cents", cent);

    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
//    match coin {
//        Coin::Quarter(state) => println!("Quarter coin from {:?} state", state),
//        _ => count += 1,
//    }

    if let Coin::Quarter(another_state) = coin {
        println!("Quarter coin from {:?} state", another_state);
    } else {
        count += 1;
    }
}
