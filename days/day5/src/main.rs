use std::fs;

fn binary_to_int(input: String) -> usize {
    return usize::from_str_radix(&input, 2).unwrap();
}

fn char_to_binary(c: char) -> char {
    if c == 'R' || c == 'B' {
        return '1';
    }
    return '0';
}

fn input_to_seat_id(input: &str) -> (usize, usize, usize) {
    let row_bin: String = input.chars().take(7).map(char_to_binary).collect();
    let row = binary_to_int(row_bin);

    let col_input = &(input.to_string())[7..10];
    let col_bin: String = col_input.chars().map(char_to_binary).collect();
    let col = binary_to_int(col_bin);

    return (row, col, row * 8 + col);
}

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Something went horribly wrong reading the file");

    /* Uncomment for part 1:
    let result = input
        .lines()
        .map(input_to_seat_id)
        .map(|(_, _, id)| id)
        .max();
    println!("{}", result);
    */

    let mut seat_ids = input
        .lines()
        .map(input_to_seat_id)
        .map(|(_, _, id)| id)
        .collect::<Vec<_>>();
    
    seat_ids.sort();
    let len = seat_ids.len();
    println!("length: {}", len);
    for idx in 1..=(len - 1) {
        let cur = seat_ids[idx];
        println!("{}", cur);
        if cur - 2 == seat_ids[idx - 1] {
            println!("Your seat is {}", cur -1);
            break;
        }
    }
}
