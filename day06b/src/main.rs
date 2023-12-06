use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

fn main() {
    number_of_ways(FILENAME);
}

fn number_of_ways(filename: &str) -> usize {
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();

    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) 
        .map(|s| s.trim()) 
        .collect::<String>()
        .parse::<u64>()  
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.trim())
        .collect::<String>()
        .parse::<u64>()  
        .unwrap();

    let count = (0..time)
        .filter(|&hold_time| distance < (time - hold_time) * hold_time)
        .count();

    println!("{}", count);

    count
}

#[cfg(test)]
mod tests {
    use crate::number_of_ways;

    #[test]
    fn test_number_of_ways() {
        assert_eq!(number_of_ways("src/test.txt"), 71503);
    }
}
