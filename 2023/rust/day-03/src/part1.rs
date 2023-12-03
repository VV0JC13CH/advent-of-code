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
        kind: char,
        score: Vec<i32>,
    }
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut line_counter = 0;

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut digits_in_line: Vec<Digit> = Vec::new();
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
                    kind: character,
                    score: vec![],
                })
            }
            char_counter += 1;
        }
        let mut collect_number: Vec<Digit> = vec![];
        if digits_in_line.len() > 0 {
            for d in 0..(digits_in_line.len()) {
                if d != 0 {
                    if digits_in_line[d].line_pos - digits_in_line[d - 1].line_pos == 1 {
                        // There is next digit in line
                        // dbg!(digits_in_line[d]);
                        collect_number.push(digits_in_line[d]);
                    } else {
                        // dbg!(digits_in_line[d]);
                        // Next digit is too far or there is no next digit in line
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
                        collect_number = vec![];
                        collect_number.push(digits_in_line[d]);
                    }
                } else {
                    // if first digit add
                    collect_number.push(digits_in_line[d]);
                }
            }
        }
        line_counter += 1;
    }
    let mut output = 0;

    for s in symbols.iter_mut() {
        for n in numbers.iter_mut() {
            if (s.line == n.line) || s.line - 1 == n.line || s.line + 1 == n.line {
                if &s.line_pos >= &(n.line_pos_start - 1) && &s.line_pos <= &(n.line_pos_end + 1) {
                    if !n.counted {
                        output += n.value;
                        n.counted = true;
                        s.score.push(n.value);
                        // dbg!(n.line, s.line);
                        // dbg!(s.kind, s.line, &s.score);
                    }
                }
            }
        }
    }
    // for s in 0..symbols.len()-1 {
    //     for n in 0..numbers.len()-1 {
    //         if (symbols[s].line == numbers[n].line) || (symbols[s].line == numbers[n].line - 1) || (symbols[s].line == numbers[n].line + 1) {
    //             // Checking if diff in lines is max 1
    //             if (symbols[s].line_pos > numbers[n].line_pos_start - 1)
    //                 && (symbols[s].line_pos < numbers[n].line_pos_end + 1 && !numbers[n].counted)
    //             {
    //                 if !numbers[n].counted {
    //                 output += numbers[n].value;
    //                 symbols[s].score.push(numbers[n].value);
    //                 numbers[n].counted = true;
    //                 dbg!(
    //                     numbers[n].line_pos_start,
    //                     numbers[n].line_pos_end,
    //                     numbers[n].line,
    //                     numbers[n].value
    //                 );
    //                 dbg!(symbols[s].line_pos, symbols[s].line, symbols[s].kind, &symbols[s].score);
    //             }}
    //         }
    //     }
    // }

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
