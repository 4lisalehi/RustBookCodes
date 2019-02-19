use std::collections::HashMap;

// These two lines will cause compile error if uncommented. Because two Enums with same name
// have been imported and cause ambiguity
//use std::fmt::Result;
//use std::io::Result;

//use std::fmt;
//use std::io;


//A better solution using renaming
use std::fmt::Result;
use std::io::Result as IoResult;

use std::{cmp::Ordering};

use std::io::{self, Write};


//fn function1 () -> Result {
//}
//
//fn function2 () -> IoResult {
//}

mod sound;

mod performance_group {
    pub use crate::sound::instrument; // *Reference line
    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}


fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map: {:?}", map);
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet(); // Because we have used "pub use" in performance_group, we can now do this.
}
