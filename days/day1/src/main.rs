use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let result = input
        .lines()
        .map(|input| input.parse::<u32>())
        .filter_map(Result::ok)
        .combinations(2) // Change to `3` for part 2
        .filter(|combination| combination.iter().sum::<u32>() == 2020)
        .next();

    match result {
        None => println!("Found nothing!"),
        Some(numbers) => println!(
            "Found {:?}, sum: {}, product: {}",
            numbers,
            numbers.iter().sum::<u32>(),
            numbers.iter().product::<u32>()
        ),
    }
}
