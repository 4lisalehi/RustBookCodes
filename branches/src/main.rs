fn hundred_fibonacci() -> u32 {
    let mut a = 0;
    let mut b = 1;
    loop {
        let temp = a;
        a = b;
        b += temp;
        if b > 100 {
            break;
        }
    }
    return b;
}


fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else if number == 0 {
        println!("condition was false");
    } else if number == 6 {
        println!("condition was false");
    }


    let a = [10, 20, 30, 40, 50, 60];
    let mut cnt = 0;

    while cnt < 5 {
        println!("{}", cnt);
        cnt += 1;
    }

    for elem in a.iter() {
        println!("The value is: {}", elem);
    }

    for number in (1..45).rev() {
        println!("{}!", number);
    }
    println!("{}", hundred_fibonacci());
}
