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
        let chars: Vec<char> = prepend_digits_to_words(ln)
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

/// This will put a digit char infront of words from left to right
/// this will not interfere with overlapping words of the lengths in question
/// And this will preserve order of words' occurence
/// PERF: Note: replacing this and the above pattern with a
/// state machine that can do ordered adds would be superior
/// Though it would still have potential issues with overlapping patterns
/// WARN: Not relevant for what we're testing for, but
/// the strategy below will be corrupt if used with left-aligned overlapping patterns
/// (e.g. "six" & "sixty") -- those are not among the patterns we're using
/// but it still bears notw
fn prepend_digits_to_words(input: &str) -> String {
    let mut output: Vec<char> = input.chars().collect();
    let mut replace_notes = Vec::<(usize, &str)>::new();
    let replace_sets = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for pat in replace_sets {
        replace_notes.extend(input.match_indices(pat));
    }
    for (idx, pat) in replace_notes {
        let add = match pat {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            _ => panic!("Could not match: {}", pat),
        };
        output[idx] = add;
    }

    output.into_iter().collect::<String>()
}

#[allow(dead_code)]
///NOTE: here as an example of a non-working approach
/// the problem weakness was noted at creation, however
/// I was surprised to see the test set use that form of input
/// takes a string reference and creates a new string with number words iteratively replaced by digits
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
