use lazy_static::lazy_static;
use regex::Regex;

pub trait Token {
    fn line(&self) -> usize;
    fn token(&self) -> &TokenType;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Atom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloxToken {
    line: usize,
    token: TokenType,
}

impl FloxToken {
    pub fn atom(&self) -> Result<String, String> {
        match &self.token {
            TokenType::Atom(s) => Ok(s.clone()),
            _ => {
                return Err(format!("Expected atom, got: {:?}", self));
            }
        }
    }
}

impl Token for FloxToken {
    fn line(self: &FloxToken) -> usize {
        self.line
    }

    fn token(self: &FloxToken) -> &TokenType {
        &self.token
    }
}

trait Scanner: Iterator {
    fn token(&mut self) -> Option<&dyn Token>;
    fn peek(&mut self) -> Option<&dyn Token>;
}

#[derive(Debug, Clone)]
pub struct FloxScanner {
    tokens: Vec<(String, usize)>,
    pos: usize,
    current_line: usize,
}

impl FloxScanner {
    pub fn new(source: &str) -> FloxScanner {
        FloxScanner {
            tokens: tokenize(source),
            pos: 0,
            current_line: 0,
        }
    }

    //pub fn previous(&self) -> Option<Box<dyn Token>> {
    //    let token: &str = &self.tokens[self.pos - 1].0;
    //    match token {
    //        "(" => Some(TokenType::LeftParen),
    //        ")" => Some(TokenType::RightParen),
    //        _ => Some(TokenType::Atom(token.to_string())),
    //    }
    //}

    pub fn get_line(&self) -> usize {
        self.current_line
    }
}

impl<'a> Iterator for FloxScanner {
    type Item = &'a dyn Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.token()?)
    }
}

impl Scanner for FloxScanner {
    fn token(&mut self) -> Option<&dyn Token> {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn peek(&mut self) -> Option<&dyn Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let (token, line) = &self.tokens[self.pos];
        let token = match &token as &str {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            _ => TokenType::Atom(token.to_string()),
        };
        self.current_line = *line;
        Some(&FloxToken {
            line: *line,
            token: token,
        })
    }
}

fn tokenize(source: &str) -> Vec<(String, usize)> {
    let source = source.replace("(", " ( ");
    let source = source.replace(")", " ) ");
    let lines = source.split('\n');
    let mut tokens: Vec<(String, usize)> = Vec::new();

    for (i, line) in lines.enumerate() {
        for tok in line.split_whitespace() {
            tokens.push((tok.to_string(), i + 1));
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut scanner = FloxScanner::new("");
        assert_eq!(scanner.token(), None);
    }

    #[test]
    fn test_tokenize() {
        let mut scan = FloxScanner::new("(+ 1 2)");
        assert_eq!(scan.token().unwrap(), (TokenType::LeftParen, 1));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("+".to_string()), 1));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("1".to_string()), 1));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("2".to_string()), 1));
        assert_eq!(scan.token().unwrap(), (TokenType::RightParen, 1));
    }

    #[test]
    fn test_tokenize_with_new_lines() {
        let mut scan = FloxScanner::new("(+ 1 2)\n(* 2 3)");
        assert_eq!(scan.token().unwrap(), (TokenType::LeftParen, 1));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("+".to_string()), 1));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("1".to_string()), 1));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("2".to_string()), 1));
        assert_eq!(scan.token().unwrap(), (TokenType::RightParen, 1));
        assert_eq!(scan.token().unwrap(), (TokenType::LeftParen, 2));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("*".to_string()), 2));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("2".to_string()), 2));
        assert_eq!(scan.token().unwrap(), (TokenType::Atom("3".to_string()), 2));
        assert_eq!(scan.token().unwrap(), (TokenType::RightParen, 2));
    }
}
