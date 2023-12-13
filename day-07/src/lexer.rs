use derive_more::Unwrap;
use logos::{Lexer, Logos};

fn to_arr(lex: &mut Lexer<Token>) -> Option<[Card; 5]> {
        let slice = lex.slice();
        let out = slice
                .chars()
                .enumerate()
                .fold([Card::INIT; 5], |mut acc, (id, c)| {
                        acc[id] = Card::from_char(&c).expect("bad parse");
                        acc
                });
        Some(out)
}

/// Token Types for Day 07 Input
#[derive(Logos, Unwrap, Debug, PartialEq)]
pub enum Token {
        #[regex(r"[ \n]+", logos::skip)]
        Ignored,

        #[regex(r"[AKQJT98765432]{5}", to_arr)]
        ProtoHand([Card; 5]),

        #[regex(r"[0-9]+", |lex| lex.slice().parse::<u64>().expect("parse failure"))]
        Bid(u64),
}

/// Card Types
/// note: derived order is asscending from top to bottom as written.
///       (e.g. here, c2 < c3 < c4 < ... < cA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
        INIT,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        CT,
        CJ,
        CQ,
        CK,
        CA,
}

impl Card {
        pub fn from_char(c: &char) -> Option<Card> {
                match c {
                        '2' => Some(Card::C2),
                        '3' => Some(Card::C3),
                        '4' => Some(Card::C4),
                        '5' => Some(Card::C5),
                        '6' => Some(Card::C6),
                        '7' => Some(Card::C7),
                        '8' => Some(Card::C8),
                        '9' => Some(Card::C9),
                        'T' => Some(Card::CT),
                        'J' => Some(Card::CJ),
                        'Q' => Some(Card::CQ),
                        'K' => Some(Card::CK),
                        'A' => Some(Card::CA),
                        _ => None,
                }
        }
}
