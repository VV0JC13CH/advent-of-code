use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    let (max_red, max_green, max_blue) = (12, 13, 14);
    let output = input
        .lines()
        .map(|line| {
            let l = line.split(": ").collect::<Vec<_>>();
            let game_id = l[0].strip_prefix("Game ").unwrap_or("0").parse::<i32>();
            let game_results: Vec<&str> = l[1].split(";").collect();
            let mut impossible = false;
            for result in game_results {
                let colors: Vec<&str> = result.split(",").collect();
                for color in colors {
                    let c_data: Vec<&str> = color.trim_start().split(" ").collect();
                    let amount: i32 = c_data[0].parse::<i32>().expect("Couldn't parse the amount");
                    if !impossible {
                    impossible = match c_data[1] {
                        "red" => {
                            if amount > max_red {
                                true
                            } else {
                                false
                            }
                        }
                        "blue" => {
                            if amount > max_blue {
                                true
                            } else {
                                false
                            }
                        }
                        "green" => {
                            if amount > max_green {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }}
                }
            }

            if impossible {
                0
            } else {
                game_id.unwrap_or(0)
            }
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
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
