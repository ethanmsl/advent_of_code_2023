use anyhow::Result;
use logos::{Lexer, Logos};
use tracing::{event, Level};

// Note: callbacks can return `Option` or `Result`
#[tracing::instrument]
fn kilo(lex: &mut Lexer<Token>) -> Option<u64> {
        let slice = lex.slice();
        let n: u64 = slice[..slice.len() - 1].parse()
                                             .ok()?; // skip 'k'
        Some(n * 1_000)
}

#[tracing::instrument]
fn mega(lex: &mut Lexer<Token>) -> Option<u64> {
        let slice = lex.slice();
        let n: u64 = slice[..slice.len() - 1].parse()
                                             .ok()?; // skip 'm'
        Some(n * 1_000_000)
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
        // Callbacks can use closure syntax, or refer
        // to a function defined elsewhere.
        //
        // Each pattern can have its own callback.
        #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
        #[regex("[0-9]+k", kilo)]
        #[regex("[0-9]+m", mega)]
        Number(u64),
}

#[tracing::instrument]
fn main() -> Result<()> {
        tracing_subscriber::fmt::init();

        event!(Level::ERROR, "starting");
        event!(Level::WARN, "starting");
        event!(Level::INFO, "starting");
        event!(Level::DEBUG, "starting");
        event!(Level::TRACE, "starting");

        let lex = Token::lexer("5 42k 75m");

        for tok in lex.spanned() {
                println!("{:?}", tok);
        }
        Ok(())
}
