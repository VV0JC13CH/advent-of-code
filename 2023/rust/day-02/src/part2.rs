use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let l = line.split(": ").collect::<Vec<_>>();
            let game_results: Vec<&str> = l[1].split(";").collect();
            let mut score = 0;
            let (mut max_red, mut max_blue, mut max_green) = (1, 1, 1);
            for result in game_results {
                let colors: Vec<&str> = result.split(",").collect();
                for color in colors {
                    let c_data: Vec<&str> = color.trim_start().split(" ").collect();
                    let amount: i32 = c_data[0].parse::<i32>().expect("Couldn't parse the amount");
                    match c_data[1] {
                        "red" => {
                            if amount > max_red {
                                max_red = amount
                            }
                        }
                        "blue" => {
                            if amount > max_blue {
                                max_blue = amount
                            }
                        }
                        "green" => {
                            if amount > max_green {
                                max_green = amount
                            }
                        }
                        _ => {}
                    }
                    score = max_red * max_green * max_blue
                }

            }
            score
        })
        .sum::<i32>();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
