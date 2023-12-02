use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";

fn main() {
    sum_calibration_values(FILENAME);
}

fn sum_calibration_values(filename: &str) -> u32 {
    let sum = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();

    println!("Sum: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use crate::sum_calibration_values;

    #[test]
    fn test_sum_calibration_values() {
        let filename = "src/test.txt";
        let sum = sum_calibration_values(filename);
        assert_eq!(sum, 142);
    }
}
