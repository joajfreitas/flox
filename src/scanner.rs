use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Atom(String),
}

impl TokenType {
    pub fn atom(&self) -> Result<String, String> {
        match self {
            TokenType::Atom(s) => Ok(s.clone()),
            _ => {
                return Err(format!("Expected atom, got: {:?}", self));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token: TokenType,
    line: usize,
}

impl Token {
    pub fn atom(&self) -> Result<String, String> {
        self.token.atom()
    }

    pub fn type(&self) -> TokenType {
        self.token
    }
}


#[derive(Debug, Clone)]
pub struct Scanner {
    tokens: Vec<(String, usize)>,
    pos: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            tokens: tokenize(source),
            pos: 0,
        }
    }

    pub fn scan(&mut self) -> Option<Token> {
        let token = self.peek();
        self.pos += 1;
        token
    }

    pub fn peek(&self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let token: &str = &self.tokens[self.pos];
        Some(match token {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            _ => TokenType::Atom(token.to_string()),
        })
    }

    pub fn previous(&self) -> Option<Token> {
        let token: &str = &self.tokens[self.pos - 1];
        match token {
            "(" => Some(TokenType::LeftParen),
            ")" => Some(TokenType::RightParen),
            _ => Some(TokenType::Atom(token.to_string())),
        }
    }

    pub fn next_tokens(&self) -> Vec<String> {
        self.tokens[self.pos..].to_vec()
    }
}

fn tokenize(source: &str) -> Vec<(String, usize)> {
    let source = source.replace("(", " ( ");
    let source = source.replace(")", " ) ");
    let lines = source.split('\n');
    let mut tokens: Vec<String> = Vec::new();

    for line in lines {
        for tok in line.split_whitespace() {
            tokens.push(tok.to_string());
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut scanner = Scanner::new("");
        assert_eq!(scanner.scan(), None);
    }

    #[test]
    fn test_tokenize() {
        let mut scan = Scanner::new("(+ 1 2)");
        assert_eq!(scan.scan().unwrap().type(), TokenType::LeftParen);
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("+".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("1".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("2".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::RightParen);
    }

    #[test]
    fn test_tokenize_with_new_lines() {
        let mut scan = Scanner::new("(+ 1 2)\n(* 2 3)");
        assert_eq!(scan.scan().unwrap().type(), TokenType::LeftParen);
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("+".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("1".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("2".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::RightParen);
        assert_eq!(scan.scan().unwrap().type(), TokenType::LeftParen);
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("*".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("2".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::Atom("3".to_string()));
        assert_eq!(scan.scan().unwrap().type(), TokenType::RightParen);
    }
}
