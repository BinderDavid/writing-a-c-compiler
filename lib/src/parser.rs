use crate::{
    ast::{Exp, FunctionDefinition, Program, Statement},
    lexer::Token,
};

type Error = String;

pub fn parse_program(tokens: &mut Vec<Token>) -> Result<Program, Error> {
    let fundef = parse_fundef(tokens)?;
    if !tokens.is_empty() {
        Err("Extra tokens at end of file".to_string())
    } else {
        Ok(Program { ef: fundef })
    }
}

pub fn parse_fundef(tokens: &mut Vec<Token>) -> Result<FunctionDefinition, Error> {
    expect_token(tokens, Token::Int)?;
    let id = expect_ident(tokens)?;
    expect_token(tokens, Token::OpenParen)?;
    expect_token(tokens, Token::Void)?;
    expect_token(tokens, Token::CloseParen)?;
    expect_token(tokens, Token::OpenBrace)?;
    let stmt = parse_statement(tokens)?;
    expect_token(tokens, Token::CloseBrace)?;
    Ok(FunctionDefinition { name: id, body: stmt })
}

pub fn parse_statement(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
    let tok = get_token(tokens)?;
    match tok {
        Token::Return => {
            let exp = parse_exp(tokens)?;
            expect_token(tokens, Token::Semi)?;
            Ok(Statement::Return(exp))
        }
        tok => Err(format!("Unexpected token in statement: {}", tok)),
    }
}

pub fn parse_exp(tokens: &mut Vec<Token>) -> Result<Exp, Error> {
    let tok = get_token(tokens)?;
    match tok {
        Token::IntLit(i) => Ok(Exp::Constant(i)),
        tok => Err(format!("Unexpected token in expression: {}", tok)),
    }
}

pub fn get_token(tokens: &mut Vec<Token>) -> Result<Token, Error> {
    if tokens.is_empty() {
        Err("No more tokens".to_string())
    } else {
        let fst = tokens.remove(0);
        Ok(fst)
    }
}

pub fn expect_token(tokens: &mut Vec<Token>, expected: Token) -> Result<(), Error> {
    let tok = get_token(tokens)?;
    if tok == expected {
        Ok(())
    } else {
        Err(format!("Expected token: {}, but found: {}", expected, tok))
    }
}

pub fn expect_ident(tokens: &mut Vec<Token>) -> Result<String, Error> {
    let tok = get_token(tokens)?;
    match tok {
        Token::Ident(id) => Ok(id),
        tok => Err(format!("Expected identifier, got: {}", tok)),
    }
}
