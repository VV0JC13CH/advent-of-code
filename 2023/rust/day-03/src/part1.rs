use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    #[derive(Debug)]
    struct Number {
        line_pos_start: i32,
        line_pos_end: i32,
        line: i32,
        value: i32,
        counted: bool,
    }
    #[derive(PartialEq, Copy, Clone, Debug)]
    struct Digit {
        line_pos: i32,
        value: char,
    }
    #[derive(Debug)]
    struct Symbol {
        line_pos: i32,
        line: i32,
        score: Vec<i32>,
    }
    let mut numbers: Vec<Number> = Vec::new();
    let mut digits_in_line: Vec<Digit> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut line_counter = 0;

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut char_counter = 0;
        let chars = line.chars();

        for character in chars {
            if character.is_digit(10) {
                digits_in_line.push(Digit {
                    line_pos: char_counter,
                    value: character,
                })
            }
            if !character.is_digit(10) && character != '.' {
                symbols.push(Symbol {
                    line: line_counter,
                    line_pos: char_counter,
                    score: vec![],
                })
            }
            char_counter += 1;
        }
        let mut collect_number: Vec<Digit> = vec![];
        for d in 0..digits_in_line.len() - 1 {
            if digits_in_line[d + 1].line_pos - digits_in_line[d].line_pos == 1 {
                // There is next digit in line
                collect_number.push(digits_in_line[d]);
            } else if digits_in_line[d + 1].line_pos - digits_in_line[d].line_pos != 1
                || digits_in_line[d].line_pos == digits_in_line[digits_in_line.len() - 1].line_pos
            {
                // Next digit is too far or there is no next digit in line
                collect_number.push(digits_in_line[d]);
                let start_pos = collect_number[0].line_pos;
                let end_pos = collect_number[collect_number.len() - 1].line_pos;
                let mut value: String = Default::default();
                for dig in collect_number {
                    value.push(dig.value)
                }
                numbers.push(Number {
                    line_pos_start: start_pos,
                    line_pos_end: end_pos,
                    counted: false,
                    value: value.parse::<i32>().expect("Couldn't parse"),
                    line: line_counter,
                });
                collect_number = vec![]
            } else {
                collect_number = vec![]
            }
        }
        line_counter += 1;
    }
    let mut output = 0;
    for s in 0..symbols.len()-1 {
        for n in 0..numbers.len()-1 {
            if (symbols[s].line - numbers[n].line).abs() <= 1 {
                // Checking if diff in lines is max 1
                if (symbols[s].line_pos > numbers[n].line_pos_start - 1)
                    && (symbols[s].line_pos < numbers[n].line_pos_end + 1 && !numbers[n].counted)
                {
                    if !symbols[s].score.contains(&numbers[n].value) {
                    output += numbers[n].value;
                    symbols[s].score.push(numbers[n].value);
                    dbg!(
                        numbers[n].line_pos_start,
                        numbers[n].line_pos_end,
                        numbers[n].line,
                        numbers[n].value
                    );
                    dbg!(symbols[s].line_pos, symbols[s].line, &symbols[s].score);
                }}
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
