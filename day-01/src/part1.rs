use crate::custom_error::AocError;

/// Take first and last digit char (may be the same!)
/// from each line to form a number.
/// Return that sum.
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let mut nums = Vec::<u32>::new();

    // lines
    for ln in input.lines() {
        // ascii digits
        let chars: Vec<char> = ln.chars().filter(|c| c.is_ascii_digit()).collect();

        // extracting, formatting, parsing, pushing
        if let (Some(&first), Some(&last)) = (chars.first(), chars.last()) {
            let str_num = format!("{}{}", first, last);
            match str_num.parse::<u32>() {
                Ok(num) => nums.push(num),
                Err(_) => panic!("Could not parse: {} \nfrom line: {}", str_num, ln),
            }
        }
    }

    Ok(nums.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(input)?, 142);
        Ok(())
    }
}
