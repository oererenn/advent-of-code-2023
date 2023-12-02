use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";

fn main() {
    sum_calibration_values(FILENAME);
}

fn sum_calibration_values(filename: &str) -> u32 {
    let sum = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
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
        let result = sum_calibration_values("src/test.txt");
        assert_eq!(result, 281);
    }
}
