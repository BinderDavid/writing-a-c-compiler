use std::fmt;

use logos::Logos;

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
#[logos(skip r"\s*", skip r"[ \t\n\f]+", error = LexicalError)]
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
    #[token("~")]
    Tilde,
    #[token("-")]
    Hyphen,
    #[token("--")]
    DoubleHyphen,
    // Identifiers
    //
    //
    #[regex(r"[a-zA-Z_]\w*", |lex| lex.slice().to_string())]
    Ident(String),
    // See: https://github.com/maciejhirsz/logos/issues/415
    #[regex(r"[0-9]+[a-zA-Z_]+", |_| None)]
    BadIdentifier,
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

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let res: Result<Vec<Token>, LexicalError> = Token::lexer(input).collect();
    res.map_err(|err| format!("{err}"))
}
