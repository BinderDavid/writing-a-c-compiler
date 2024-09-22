use std::fmt;

use logos::{Logos, SpannedIter};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"\s*", skip r"--(([^ \n\r]| [^\|\n\r])[^\n\r]*)?[\n\r]*", error = LexicalError)]
pub enum Token {
    // Keywords
    //
    //
    #[token("int")]
    Int,
    #[token("return")]
    Return,
    #[token("void")]
    Void,
    // Symbols
    //
    //
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token(";")]
    Semi,
    // Identifiers
    //
    //
    #[regex(r"[a-zA-Z_]+", |lex| lex.slice().to_string())]
    Ident(String),
    // Literals
    //
    //
    #[regex(r"0|[1-9][0-9]*", |lex| lex.slice().parse::<i64>().unwrap())]
    IntLit(i64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self { token_stream: Token::lexer(input).spanned() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}
