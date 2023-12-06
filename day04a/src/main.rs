use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";

fn main() {
    sum_points(FILENAME);
}

fn sum_points(filename: &str) -> u32 {
    let sum = read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning, ours) = numbers.split_once('|').unwrap();

            let winning_numbers: Vec<u32> = winning
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            let our_numbers: Vec<u32> = ours
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            let matches = winning_numbers
                .iter()
                .filter(|&win_num| our_numbers.contains(win_num))
                .count();

            if matches > 0 {
                1 << (matches - 1)
            } else {
                0
            }
        })
        .sum();
    println!("{}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use crate::sum_points;

    #[test]
    fn test_sum_points() {
        let filename = "src/test.txt";
        let sum = sum_points(filename);
        assert_eq!(sum, 13);
    }
}
