use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn main() {
    multiply_number_of_ways(FILENAME);
}

fn multiply_number_of_ways(filename: &str) -> usize {
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();

    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // Skip the "Time:" label
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // Skip the "Distance:" label
        .map(|s| s.parse().unwrap())
        .collect();

    let races = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance });

    let product = races
        .map(|race| {
            (0..race.time)
                .filter(|&hold_time| race.distance < (race.time - hold_time) * hold_time)
                .count()
        })
        .product();

    println!("{}", product);

    product
}

#[cfg(test)]
mod tests {
    use crate::multiply_number_of_ways;

    #[test]
    fn test_multiply_number_of_ways() {
        assert_eq!(multiply_number_of_ways("src/test.txt"), 288);
    }
}
