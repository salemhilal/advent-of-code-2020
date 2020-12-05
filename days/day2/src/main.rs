use std::fs;

#[derive(Debug, PartialEq)]
struct Password<'a> {
    c: char,
    low: u32,
    high: u32,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.c).count() as u32;
        return count <= self.high && count >= self.low;
    }

    fn is_valid_part_2(&self) -> bool {
        let first = self.password.chars().nth((self.low - 1) as usize).unwrap() == self.c;
        let second = self.password.chars().nth((self.high - 1) as usize).unwrap() == self.c;
        return first ^ second;
    }
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(input: &'a str) -> Self {
        let pieces: Vec<_> = input.split(":").collect();

        assert_eq!(pieces.len(), 2);
        let password = pieces[1].trim();

        let rule_pieces: Vec<_> = pieces[0].trim().split(" ").collect();
        assert_eq!(rule_pieces.len(), 2);

        let rule_char = rule_pieces[1]
            .parse::<char>()
            .expect("Couldn't parse rule char");

        let rule_range: Vec<_> = rule_pieces[0].split("-").collect();
        assert_eq!(rule_range.len(), 2);

        let rule_low = rule_range[0]
            .parse::<u32>()
            .expect("Couldn't parse ruleLow");
        let rule_high = rule_range[1]
            .parse::<u32>()
            .expect("Couldn't parse ruleHigh");

        return Password {
            c: rule_char,
            low: rule_low,
            high: rule_high,
            password: password,
        };
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let result = input
        .lines()
        .map(Password::from)
        .filter(|password| password.is_valid()) // change to is_valid_part_2 for part 2
        .collect::<Vec<_>>()
        .len();

    println!("There are {} invalid passwords", result);
}
