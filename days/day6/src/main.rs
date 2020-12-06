#![feature(iterator_fold_self)]

use array_tool::vec::Intersect;
use itertools::Itertools;
use std::fs;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    return input
        .split("\n\n")
        .map(|lines| lines.replace("\n", ""))
        .map(|lines| lines.chars().unique().collect::<Vec<_>>().len())
        .sum();
}

fn part_2(input: &str) -> usize {
    return input
        .split("\n\n")
        .map(|line| {
            return line
                .split("\n")
                .filter(|line| line.len() != 0)
                .map(|line| line.chars().collect::<Vec<_>>())
                .fold_first(|acc, next| acc.intersect(next))
                .unwrap().len();
        })
        .sum::<usize>();
}

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Something went horribly wrong reading the file");

    let results = part_2(&input); // Change this to part_1 for part 1

    println!("{:?}", results);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(""), 0);
        assert_eq!(part_1("abcdefg"), 7);
        assert_eq!(part_1("abc\nacb\nbac"), 3);
        assert_eq!(part_1("abc\nbac"), 3);
        assert_eq!(part_1("abc"), 3);
        assert_eq!(part_1("abc\ndef\nghi"), 9);
        assert_eq!(part_1("abc\nde\nf"), 6);

        assert_eq!(part_1("abc\ndef\nghi\n\na\nb\nc"), 12);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
        assert_eq!(part_2("abcdefg"), 7);
        assert_eq!(part_2("abc\nacb\nbac"), 3);
        assert_eq!(part_2("abc\nbac"), 3);
        assert_eq!(part_2("abc"), 3);

        assert_eq!(part_2("abc\ndef\nghi"), 0);
        assert_eq!(part_2("abc\nde\nf"), 0);

        assert_eq!(part_2("abc\ndef\nghi\n\na\nb\nc"), 0);
        assert_eq!(part_2("abc\nad\na"), 1);
        assert_eq!(part_2("abc\nad\na\n\ngs\nsg\ngs"), 3);
        assert_eq!(part_2("ab\nabc\ncab"), 2);
        assert_eq!(part_2("ab\nab\nab\nab\nab\nab\nab"), 2);
        assert_eq!(part_2("ab\nab\nab\nab\nab\nab\na"), 1);
    }
}
