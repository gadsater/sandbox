/// Token types for the scripting language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),

    // Identifiers & keywords
    Ident(String),
    Let,
    Def,
    Fun,
    Ret,
    On,
    Tup,
    SelfKw,

    // Type annotation separator
    ColonColon, // ::

    // Symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Colon,
    Eq,        // =
    PlusEq,    // +=
    MinusEq,   // -=
    StarEq,    // *=
    SlashEq,   // /=
    FatArrow,  // =>

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,   // ==
    BangEq, // !=
    Lt,
    Gt,
    Le,
    Ge,
    And, // and
    Or,  // or

    Newline,
    Eof,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while self.pos < self.chars.len() {
            self.skip_spaces();
            if self.pos >= self.chars.len() {
                break;
            }
            let ch = self.chars[self.pos];

            // Comments: // until end of line
            if ch == '/' && self.peek(1) == Some('/') {
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                    self.pos += 1;
                }
                continue;
            }

            match ch {
                '\n' | '\r' => {
                    // Collapse consecutive newlines into one token.
                    while self.pos < self.chars.len()
                        && (self.chars[self.pos] == '\n' || self.chars[self.pos] == '\r')
                    {
                        self.pos += 1;
                    }
                    tokens.push(Token::Newline);
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.pos += 1;
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.pos += 1;
                }
                '{' => {
                    tokens.push(Token::LBrace);
                    self.pos += 1;
                }
                '}' => {
                    tokens.push(Token::RBrace);
                    self.pos += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.pos += 1;
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.pos += 1;
                }
                ':' => {
                    if self.peek(1) == Some(':') {
                        tokens.push(Token::ColonColon);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Colon);
                        self.pos += 1;
                    }
                }
                '=' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::EqEq);
                        self.pos += 2;
                    } else if self.peek(1) == Some('>') {
                        tokens.push(Token::FatArrow);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Eq);
                        self.pos += 1;
                    }
                }
                '+' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::PlusEq);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Plus);
                        self.pos += 1;
                    }
                }
                '-' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::MinusEq);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Minus);
                        self.pos += 1;
                    }
                }
                '*' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::StarEq);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Star);
                        self.pos += 1;
                    }
                }
                '/' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::SlashEq);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Slash);
                        self.pos += 1;
                    }
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.pos += 1;
                }
                '!' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::BangEq);
                        self.pos += 2;
                    } else {
                        return Err(format!("unexpected character '!' at position {}", self.pos));
                    }
                }
                '<' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::Le);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Lt);
                        self.pos += 1;
                    }
                }
                '>' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::Ge);
                        self.pos += 2;
                    } else {
                        tokens.push(Token::Gt);
                        self.pos += 1;
                    }
                }
                '"' => {
                    tokens.push(self.read_string()?);
                }
                _ if ch.is_ascii_digit() => {
                    tokens.push(self.read_number());
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    tokens.push(self.read_ident_or_keyword());
                }
                _ => {
                    return Err(format!("unexpected character '{ch}' at position {}", self.pos));
                }
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    fn skip_spaces(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos] == ' '
            || self.pos < self.chars.len() && self.chars[self.pos] == '\t'
        {
            self.pos += 1;
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.pos += 1; // skip opening quote
        let mut s = String::new();
        while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
            if self.chars[self.pos] == '\\' {
                self.pos += 1;
                if self.pos >= self.chars.len() {
                    return Err("unterminated string escape".into());
                }
                match self.chars[self.pos] {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    c => s.push(c),
                }
            } else {
                s.push(self.chars[self.pos]);
            }
            self.pos += 1;
        }
        if self.pos >= self.chars.len() {
            return Err("unterminated string literal".into());
        }
        self.pos += 1; // skip closing quote
        Ok(Token::Str(s))
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        let mut is_float = false;
        while self.pos < self.chars.len()
            && (self.chars[self.pos].is_ascii_digit() || self.chars[self.pos] == '.')
        {
            if self.chars[self.pos] == '.' {
                if is_float {
                    break;
                }
                // Check that the next character is a digit (otherwise it's field access).
                if self.peek(1).map_or(true, |c| !c.is_ascii_digit()) {
                    break;
                }
                is_float = true;
            }
            self.pos += 1;
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        if is_float {
            Token::Float(text.parse().unwrap())
        } else {
            Token::Int(text.parse().unwrap())
        }
    }

    fn read_ident_or_keyword(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.chars.len()
            && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_')
        {
            self.pos += 1;
        }
        let word: String = self.chars[start..self.pos].iter().collect();
        match word.as_str() {
            "let" => Token::Let,
            "def" => Token::Def,
            "fun" => Token::Fun,
            "ret" => Token::Ret,
            "on" => Token::On,
            "tup" => Token::Tup,
            "self" => Token::SelfKw,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "and" => Token::And,
            "or" => Token::Or,
            _ => Token::Ident(word),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_let_binding() {
        let tokens = Lexer::new("let x = 3").tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("x".into()),
                Token::Eq,
                Token::Int(3),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn lex_fun_decl() {
        let tokens = Lexer::new("let y :: fun (x int, y int) int =")
            .tokenize()
            .unwrap();
        assert!(tokens.contains(&Token::ColonColon));
        assert!(tokens.contains(&Token::Fun));
    }
}
