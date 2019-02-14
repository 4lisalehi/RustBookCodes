fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

fn takes_ownership(a_string: String) {
    println!("{}", a_string);
}

fn gives_ownership() -> String{
    let some_string = String::from("here i come");
    some_string
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


fn main() {

    let s1 = gives_ownership();
    // s1 is valid here

    takes_ownership(s1); // The the functions takes ownership of s1 and s1 is no longer valid

    let s2 = String::from("Sample string");
    let s3 = takes_and_gives_back_ownership(s2);

    // s2 is no longer valid
    // but s3 is and has gotten the ownership back from the function

    let mut my_string = String::from("test string");
    change(&mut my_string); // ERROR: compile error because pointers are by default immutable and we cannot mutate them

    // In this part, we check the multiple reference
    let mut some_string = String::from("some string here");
    let p1 = &mut some_string;
    let p2 = &mut some_string;
}// Here, at the end of the scope, s3 calls drop, s1 & s2 have already been invalidated
