//Exercise: Calculate the median and mode of a list of numbers using Rust's standard collections.
use std::collections::HashMap;

fn calculate_median(numbers: &Vec<i32>) -> f32 {
    let mut sorted_numbers = numbers.clone();

    //Check https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable
    sorted_numbers.sort_unstable();

    let len = sorted_numbers.len();
    if len % 2 == 0 {
        let mid1 = sorted_numbers[len / 2 - 1];
        let mid2 = sorted_numbers[len / 2];
        (mid1 as f32 + mid2 as f32) / 2.0
    } else {
        sorted_numbers[len / 2] as f32
    }
}

fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();

    for &number in numbers {
        *frequency_map.entry(number).or_insert(0) += 1;
    }

    let mut mode = numbers[0];
    let mut max_count = 0;

    for (&number, &count) in &frequency_map {
        if count > max_count {
            max_count = count;
            mode = number;
        }
    }

    mode
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4,-2, 5, 5, 5,-10, 3, 4, 4, 4];

    let median = calculate_median(&numbers);
    println!("Median: {}", median);

    let mode = calculate_mode(&numbers);
    println!("Mode: {}", mode);
}