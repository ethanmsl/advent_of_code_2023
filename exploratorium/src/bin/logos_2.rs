use anyhow::Result;
use logos::Logos;

#[derive(Debug, PartialEq, Clone, Default)]
enum LexingError {
        NumberParseError,
        #[default]
        Other,
}

impl From<std::num::ParseIntError> for LexingError {
        fn from(_: std::num::ParseIntError) -> Self {
                LexingError::NumberParseError
        }
}

impl From<std::num::ParseFloatError> for LexingError {
        fn from(_: std::num::ParseFloatError) -> Self {
                LexingError::NumberParseError
        }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
enum Example {
        #[regex(r"[ \n\t\f]+", logos::skip)]
        Ignored,

        #[regex("-?[0-9]+", |lex| lex.slice().parse())]
        Integer(i64),

        #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
        Float(f64),
}

#[allow(clippy::approx_constant)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
        let tokens: Vec<_> = Example::lexer("42 3.14 -5 f").spanned().collect();

        assert_eq!(
                tokens,
                &[
                        (Ok(Example::Integer(42)), 0..2),
                        (Ok(Example::Float(3.14)), 3..7),
                        (Ok(Example::Integer(-5)), 8..10),
                        (Err(LexingError::Other), 11..12), // 'f' is not a recognized token
                ],
        );
        Ok(())
}
