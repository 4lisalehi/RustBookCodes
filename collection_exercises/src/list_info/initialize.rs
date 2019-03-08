use std::io;
use std::collections::HashMap;

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let vector_length: i32 = numbers.len() as i32;
    let res = *numbers
        .get((vector_length / 2) as usize)
        .expect("Vector access failed");
    res
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    let mut frequent = -1;
    let mut max_so_far = -1;
    for (&k, v) in map {
        if v > max_so_far {
            max_so_far = v;
            frequent = k;
        }
    }
    frequent
}

fn average(numbers: &Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().sum();
    let avg: i32= sum / numbers.len() as i32;
    avg
}

fn average_using_fold(numbers: &Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().fold(0, |acc, cur| acc + cur);
    let avg = sum / numbers.len() as i32;
    avg
}

pub fn run() {
    let scanner = io::stdin();
    let mut numbers: Vec<i32> = Vec::new();
    let mut input_line = String::new();

    scanner
        .read_line(&mut input_line)
        .expect("IO_ERROR");

    numbers = input_line
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Parse error"))
        .collect();

    let avg1 = average(&numbers);
    let avg2 = average_using_fold(&numbers);
    let median = median(&mut numbers);
    let mode = mode(&numbers);

    assert_eq!(avg1, avg2);

    println!("avg1: {}, avg2: {}, median: {}, mode: {}", avg1, avg2, median, mode);
}
