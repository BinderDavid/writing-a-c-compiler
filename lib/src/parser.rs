use crate::{
    ast::{Exp, FunctionDefinition, Program, Statement},
    lexer::Token,
};

type Error = String;

pub fn parse_program(tokens: &mut [Token]) -> Result<Program, Error> {
    let fundef = parse_fundef(tokens)?;
    Ok(Program { ef: fundef })
}

pub fn parse_fundef(tokens: &mut [Token]) -> Result<FunctionDefinition, Error> {
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

pub fn parse_statement(tokens: &mut [Token]) -> Result<Statement, Error> {
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

pub fn parse_exp(tokens: &mut [Token]) -> Result<Exp, Error> {
    let tok = get_token(tokens)?;
    match tok {
        Token::IntLit(i) => Ok(Exp::Constant(i)),
        tok => Err(format!("Unexpected token in expression: {}", tok)),
    }
}

pub fn get_token(tokens: &mut [Token]) -> Result<Token, Error> {
    match tokens.first() {
        Some(tok) => Ok(tok.clone()),
        None => Err("No more tokens".to_string()),
    }
}

pub fn expect_token(tokens: &mut [Token], expected: Token) -> Result<(), Error> {
    let tok = get_token(tokens)?;
    if tok == expected {
        Ok(())
    } else {
        Err(format!("Expected token: {}, but found: {}", expected, tok))
    }
}

pub fn expect_ident(tokens: &mut [Token]) -> Result<String, Error> {
    let tok = get_token(tokens)?;
    match tok {
        Token::Ident(id) => Ok(id),
        tok => Err(format!("Expected identifier, got: {}", tok)),
    }
}
