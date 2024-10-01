#[derive(Debug)]
pub struct Scanner {}

impl Scanner {
    pub fn new(_contents: &str) -> Self {
        Scanner {}
    }

    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String> {
        todo!()
    }
}

#[derive(Debug)] 
pub enum Token {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHY_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, START,

    BANG, BANG_EQUAL, 
    EQUAL, EQUAL_EQUAL,
    GRATER, GRATER_EQUAL,
    LESS, LESS_EQUAL,

    IDENTIFIER, STRING, NUMBER,

    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, 

    EOF
}
