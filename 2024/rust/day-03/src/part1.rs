use regex::Regex;
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    // Define a regex pattern to match valid mul(X,Y) instructions
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Initialize sum to accumulate results
    let mut total_sum = 0;

    // Iterate over all matches in the input string
    for cap in re.captures_iter(_input) {
        // Extract the two numbers (X and Y) from the match
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        total_sum += x * y;
    }

    // Output the result
    println!("The total sum of valid mul operations is: {}", total_sum);
    Ok(total_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
