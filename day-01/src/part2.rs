use crate::custom_error::AocError;
use tracing::info;

/// Take first and last digit char or digit word (may be the same!)
/// from each line to form a number.
/// Return that sum.
/// WARNING: this does NOT resolve ambiguous word cases like "oneight"
/// NOTE: hackish fix, sorted words in order of length, with hand check for no overlap
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let mut nums = Vec::<u32>::new();

    // lines
    for ln in input.lines() {
        // ascii digits
        let chars: Vec<char> = words_to_digits(ln)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();

        info!("{:?}", chars);

        // extracting, formatting, parsing, pushing
        if let (Some(&first), Some(&last)) = (chars.first(), chars.last()) {
            let str_num = format!("{}{}", first, last);
            match str_num.parse::<u32>() {
                Ok(num) => nums.push(num),
                Err(_) => panic!("Could not parse: {} \nfrom line: {}", str_num, ln),
            }
        }
    }

    info!("{:?}", nums);
    Ok(nums.into_iter().sum())
}

/// takes a string reference and creates a new string with number words iteratively replaced by
/// digits
/// WARNING: this is *iterative* "oneight" will become "1ight"
/// (vs 18, on8, or neither)
/// There are no current tests for these overlaps
/// NOTE: hackish fix, sorted in order of length, with hand check for no overlap
fn words_to_digits(input: &str) -> String {
    // making compatible with our old version
    let replace_sets = vec![
        // 5
        ("seven", "7"),
        ("eight", "8"),
        ("three", "3"),
        // 4
        ("four", "4"),
        ("five", "5"),
        ("nine", "9"),
        // 3
        ("one", "1"),
        ("two", "2"),
        ("six", "6"),
    ];

    replace_sets
        .iter()
        .fold(input.to_string(), |acc, (word, digit)| {
            acc.replace(word, digit)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();

        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(process(input)?, 281);
        Ok(())
    }
}
