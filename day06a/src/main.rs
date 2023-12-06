use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

fn main() {
    multiply_number_of_ways(FILENAME);
}

fn multiply_number_of_ways(filename: &str) -> usize {
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();

    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let product: usize = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            (0..time)
                .filter(|&hold_time| distance < (time - hold_time) * hold_time)
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
