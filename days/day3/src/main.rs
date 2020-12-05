use std::fs;

fn count_trees(input: &str, right_slope: usize, down_slope: usize) -> usize {
    let (_, trees) = input
        .lines()
        .enumerate()
        .fold((0, 0), |(col, trees), (row, line)| {
            if row % down_slope != 0 {
                return (col, trees);
            }

            let _trees = if line.chars().nth(col).unwrap() == '#' {
                trees + 1
            } else {
                trees
            };
            let _col = (col + right_slope) % line.len();

            return (_col, _trees);
        });
    return trees;
}

fn main() {
    let slopes = vec![
        (1, 1),
        (3, 1), // comment out all lines but this one for part 1
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let input = fs::read_to_string("src/input.txt")
        .expect("Something went horribly wrong reading the file");

    let product = slopes
        .iter()
        .map(|(right, down)| count_trees(&input, *right, *down))
        .product::<usize>();
        
    println!("The product is {:?}.", product);
}
