use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    multiply_number_of_ways(FILENAME);
}

fn multiply_number_of_ways(filename: &str) -> usize {
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();

    let time: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // Skip the first element
        .map(|s| s.trim()) // Trim each split string
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // Skip the first element
        .map(|s| s.trim()) // Trim each split string
        .collect::<String>()
        .parse()
        .unwrap();

    let race = Race {
        time: time,
        distance: distance,
    };

    let count = (0..race.time)
        .filter(|&hold_time| race.distance < (race.time - hold_time) * hold_time)
        .count();

    println!("{}", count);

    count
}

#[cfg(test)]
mod tests {
    use crate::multiply_number_of_ways;

    #[test]
    fn test_multiply_number_of_ways() {
        assert_eq!(multiply_number_of_ways("src/test.txt"), 71503);
    }
}
