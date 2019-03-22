pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Vec {
    fn summarize(&self) -> String {
        let mut sum: u32 = 0;
        for &elem in self {
            sum += elem;
        }
        sum
    }
}


fn main() {

}
