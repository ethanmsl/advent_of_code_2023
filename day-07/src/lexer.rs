use logos::{Lexer, Logos};

fn to_arr(lex: &mut Lexer<Token>) -> Option<[char; 5]> {
        let out = [char; 5];
        let slice = lex.slice();
        let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
        Some(n * 1_000_000)
}

#[derive(Logos, Debug, PartialEq)]
enum Tokens {
        #[regex(r"[ \n]+", logos::skip)]
        Ignored,

        #[regex(r"\w{5}", |lex| Some(['1'; 5]))]
        Hand([char; 5]),

        #[regex(r"[0-9]+", |lex| lex.slice().parse::<u64>().expect("parse failure"))]
        Bid(u64),
}
