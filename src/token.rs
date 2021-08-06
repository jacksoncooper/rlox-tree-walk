use crate::token_type::TokenType;

#[derive(Clone, Debug, PartialEq)]

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token { token_type, lexeme, line }
    }
}
