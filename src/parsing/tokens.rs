use nom::IResult;
use std::collections::btree_set::Iter;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// A tokenizer over a source of string
pub struct Tokenizer<I>
where
    I: Iterator<Item = char>,
{
    iter: I,
}

pub trait IntoTokenizer {
    type Iter: Iterator<Item = char>;

    fn into_tokens(self) -> Tokenizer<Self::Iter>;
}

impl<I: IntoIterator<Item = char>> IntoTokenizer for I {
    type Iter = <I as IntoIterator>::IntoIter;

    fn into_tokens(self) -> Tokenizer<Self::Iter> {
        Tokenizer {
            iter: self.into_iter(),
        }
    }
}

impl<I> Iterator for Tokenizer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub enum Token {
    Whitespace,
    Terminator,
    Comment(&'a str),
}

fn parse_token(src: &str) -> IResult<&str, Token> {
    todo!()
}

#[derive(Debug)]
pub struct UnknownToken {}

impl Display for UnknownToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown token encountered")
    }
}

impl Error for UnknownToken {}
