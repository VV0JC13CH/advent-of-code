#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut safe_reports = 0;
    for line in _input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let is_increasing_safely = report
            .windows(2)
            .all(|lvl| (lvl[0] <= lvl[1]) && (lvl[1] - lvl[0] >= 1 && lvl[1] - lvl[0] <= 3));
        let is_decreasing_safely = report
            .windows(2)
            .all(|lvl| (lvl[0] >= lvl[1]) && (lvl[0] - lvl[1] >= 1 && lvl[0] - lvl[1] <= 3));

        if is_decreasing_safely || is_increasing_safely {
            safe_reports += 1;
        } else {
            // Brute force
            let mut is_safe_without_one_element: bool = false;
            for i in 0..report.len() {
                println!("{}", i);
                if !is_safe_without_one_element {
                    let mut modified_report: Vec<u32> = report.clone();
                    modified_report.remove(i);
                    let is_increasing_unsafely = modified_report.windows(2).all(|lvl| {
                        (lvl[0] <= lvl[1]) && (lvl[1] - lvl[0] >= 1 && lvl[1] - lvl[0] <= 3)
                    });
                    let is_decreasing_unsafely = modified_report.windows(2).all(|lvl| {
                        (lvl[0] >= lvl[1]) && (lvl[0] - lvl[1] >= 1 && lvl[0] - lvl[1] <= 3)
                    });
                    if is_decreasing_unsafely || is_increasing_unsafely {
                        is_safe_without_one_element = true;
                        safe_reports += 1;
                    }
                }
            }
        }
    }

    Ok(safe_reports.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
                    1 2 7 8 9
                    9 7 6 2 1
                    1 3 2 4 5
                    8 6 4 4 1
                    1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
