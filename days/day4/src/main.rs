use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn is_valid_passport(fields: &Vec<(String, String)>) -> bool {
    let fields_iter = fields.iter();
    let mut map = HashMap::new();
    for (key, val) in fields_iter {
        map.insert(key, val);
    }
    let field_names = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl",
        "pid",
        // "cid" // no one's gonna miss this one.
    ];
    return field_names.iter().fold(true, |is_valid, field| {
        is_valid && map.contains_key(&field.to_string())
    });
}

fn is_valid_birth_year(input: &str) -> bool {
    return is_valid_year(input, 1920, 2002);
}

fn is_valid_issue_year(input: &str) -> bool {
    return is_valid_year(input, 2010, 2020);
}

fn is_valid_expiration_year(input: &str) -> bool {
    return is_valid_year(input, 2020, 2030);
}

fn is_valid_year(input: &str, min: usize, max: usize) -> bool {
    let parsed_year = input.parse::<usize>();
    if !parsed_year.is_ok() {
        return false;
    }
    let year_num = parsed_year.unwrap();
    if year_num < min || year_num > max {
        return false;
    }
    return true;
}

fn is_valid_height(input: &str) -> bool {
    let cm_re = Regex::new(r"(\d{3})cm").unwrap();
    let in_re = Regex::new(r"(\d{2})in").unwrap();

    if cm_re.is_match(input) {
        let cap = cm_re.captures_iter(input).next().unwrap()[1].parse::<usize>();
        if !cap.is_ok() {
            return false;
        }
        let cm = cap.unwrap();
        if cm < 150 || cm > 193 {
            return false;
        }
        return true;
    }

    if in_re.is_match(input) {
        let cap = in_re.captures_iter(input).next().unwrap()[1].parse::<usize>();
        if !cap.is_ok() {
            return false;
        }
        let inches = cap.unwrap();
        if inches < 59 || inches > 76 {
            return false;
        }
        return true;
    }
    return false;
}

fn is_valid_hair(input: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    return re.is_match(input);
}

fn is_valid_passport_id(input: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    return re.is_match(input);
}

fn is_valid_eye(input: &str) -> bool {
    return input == "amb"
        || input == "blu"
        || input == "brn"
        || input == "gry"
        || input == "grn"
        || input == "hzl"
        || input == "oth";
}

fn validate_passport(fields: &Vec<(String, String)>) -> bool {
    if !is_valid_passport(fields) {
        return false;
    }

    let fields_iter = fields.iter();
    let mut map = HashMap::new();
    for (key, val) in fields_iter {
        map.insert(key, val);
    }

    let birth_year = map.get(&"byr".to_string()).unwrap();
    if !is_valid_birth_year(birth_year) {
        return false;
    }

    let issue_year = map.get(&"iyr".to_string()).unwrap();
    if !is_valid_issue_year(issue_year) {
        return false;
    }

    let exp_year = map.get(&"eyr".to_string()).unwrap();
    if !is_valid_expiration_year(exp_year) {
        return false;
    }

    let height = map.get(&"hgt".to_string()).unwrap();
    if !is_valid_height(height) {
        return false;
    }

    let hair = map.get(&"hcl".to_string()).unwrap();
    if !is_valid_hair(hair) {
        return false;
    }

    let eye = map.get(&"ecl".to_string()).unwrap();
    if !is_valid_eye(eye) {
        return false;
    }

    let passport_id = map.get(&"pid".to_string()).unwrap();
    if !is_valid_passport_id(passport_id) {
        return false;
    }

    return true;
}

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Something went horribly wrong reading the file");

    let results = input
        .split("\n\n")
        .map(|lines| lines.replace("\n", " "))
        .map(|line| {
            line.split_whitespace()
                .map(str::to_owned)
                .map(|pair| {
                    let pieces = pair.split(":").map(str::to_owned).collect::<Vec<_>>();
                    return (pieces[0].to_owned(), pieces[1].to_owned());
                })
                .collect::<Vec<(String, String)>>()
        })
        .filter(|fields| validate_passport(fields))
        // .filter(|fields| is_valid_passport(fields)) // <- use this instead of the previous line for part 1
        .count();

    println!("{:?}", results);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_birth_year() {
        assert_eq!(is_valid_birth_year(""), false);
        assert_eq!(is_valid_birth_year("fsdoifj"), false);

        assert_eq!(is_valid_birth_year("1919"), false);
        assert_eq!(is_valid_birth_year("1920"), true);
        assert_eq!(is_valid_birth_year("2002"), true);
        assert_eq!(is_valid_birth_year("2003"), false);
    }

    #[test]
    fn test_is_valid_issue_year() {
        assert_eq!(is_valid_issue_year(""), false);
        assert_eq!(is_valid_issue_year("fsdoifj"), false);

        assert_eq!(is_valid_issue_year("2009"), false);
        assert_eq!(is_valid_issue_year("2010"), true);
        assert_eq!(is_valid_issue_year("2020"), true);
        assert_eq!(is_valid_issue_year("2021"), false);
    }

    #[test]
    fn test_is_valid_height() {
        assert_eq!(is_valid_height(""), false);
        assert_eq!(is_valid_height("sdlkfjdsl"), false);

        assert_eq!(is_valid_height("149cm"), false);
        assert_eq!(is_valid_height("150cm"), true);
        assert_eq!(is_valid_height("193cm"), true);
        assert_eq!(is_valid_height("194cm"), false);
        assert_eq!(is_valid_height("58in"), false);
        assert_eq!(is_valid_height("59in"), true);
        assert_eq!(is_valid_height("76in"), true);
        assert_eq!(is_valid_height("77in"), false);
    }

    #[test]
    fn test_is_valid_hair() {
        assert_eq!(is_valid_hair(""), false);
        assert_eq!(is_valid_hair("sdlkfjsdlfkj"), false);

        assert_eq!(is_valid_hair("#abcdef"), true);
        assert_eq!(is_valid_hair("#013579"), true);
        assert_eq!(is_valid_hair("#fed190"), true);
        assert_eq!(is_valid_hair("#fed190f"), false);
        assert_eq!(is_valid_hair("#fed9f"), false);
        assert_eq!(is_valid_hair("#ghijkl"), false);
        assert_eq!(is_valid_hair("fed190"), false);
    }

    #[test]
    fn test_is_valid_eye() {
        assert_eq!(is_valid_eye(""), false);
        assert_eq!(is_valid_eye("weofijwe"), false);

        assert_eq!(is_valid_eye("amb"), true);
        assert_eq!(is_valid_eye("blu"), true);
        assert_eq!(is_valid_eye("brn"), true);
        assert_eq!(is_valid_eye("gry"), true);
        assert_eq!(is_valid_eye("grn"), true);
        assert_eq!(is_valid_eye("hzl"), true);
        assert_eq!(is_valid_eye("oth"), true);
    }

    #[test]
    fn test_is_valid_passport_id() {
        assert_eq!(is_valid_passport_id(""), false);
        assert_eq!(is_valid_passport_id("dskjfhsdkfjh"), false);

        assert_eq!(is_valid_passport_id("023456789"), true);
        assert_eq!(is_valid_passport_id("0123456789"), false);
        assert_eq!(is_valid_passport_id("0I2345689"), false);
    }
}
