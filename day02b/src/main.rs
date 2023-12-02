use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";

fn main() {
    sum_total_power(FILENAME);
}

fn sum_total_power(filename: &str) -> u32 {
    let sum = read_to_string(filename)
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (_, contents) = line.split_once(':').unwrap();

            let (max_red, max_green, max_blue) = contents.split(';').fold(
                (0, 0, 0),
                |(mut max_red, mut max_green, mut max_blue), set| {
                    set.split(',').for_each(|color| {
                        let parts: Vec<&str> = color.trim().split_whitespace().collect();
                        if let Some((num, color)) = parts
                            .get(0)
                            .and_then(|n| n.parse::<u32>().ok())
                            .zip(parts.get(1))
                        {
                            match *color {
                                "red" => max_red = max_red.max(num),
                                "green" => max_green = max_green.max(num),
                                "blue" => max_blue = max_blue.max(num),
                                _ => {}
                            };
                        };
                    });
                    (max_red, max_green, max_blue)
                },
            );

            Some(max_red * max_green * max_blue)
        })
        .sum();

    println!("Sum: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use crate::sum_total_power;

    #[test]
    fn test_sum_total_power() {
        assert_eq!(sum_total_power("src/test.txt"), 2286);
    }
}
