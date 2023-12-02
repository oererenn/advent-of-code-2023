use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";

fn main() {
    sum_possible_games(FILENAME);
}

fn sum_possible_games(filename: &str) -> u32 {
    let sum = read_to_string(filename)
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (game, contents) = line.split_once(':').unwrap();
            let game_id = game
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let is_possible_game = contents.split(';').all(|set| {
                let (mut red, mut green, mut blue) = (0, 0, 0);
                set.split(',').for_each(|color| {
                    let parts: Vec<&str> = color.trim().split_whitespace().collect();
                    if let Some((num, color)) = parts
                        .get(0)
                        .and_then(|n| n.parse::<u32>().ok())
                        .zip(parts.get(1))
                    {
                        match *color {
                            "red" => red += num,
                            "green" => green += num,
                            "blue" => blue += num,
                            _ => (),
                        }
                    }
                });
                red <= 12 && green <= 13 && blue <= 14
            });

            if is_possible_game {
                Some(game_id)
            } else {
                None
            }
        })
        .sum();

    println!("Sum: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use crate::sum_possible_games;

    #[test]
    fn test_sum_calibration_values() {
        assert_eq!(sum_possible_games("src/test.txt"), 8);
    }
}
