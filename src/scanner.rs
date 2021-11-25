use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Atom(String)
}

impl Token {
    pub fn atom(&self) -> String {
        match self {
            Token::Atom(s) => s.clone(),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scanner {
    tokens: Vec<String>,
    pos: usize,
}

impl Scanner{
    pub fn new(source: &str) -> Scanner {
        Scanner {
            tokens: tokenize(source),
            pos: 0,
        }
    }

    pub fn scan(&mut self) -> Option<Token> {
        let token = self.peek();
        self.pos = self.pos + 1;
        token
    }

    pub fn peek(&self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let token: &str = &self.tokens[self.pos];
        Some(match token {
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            _ => Token::Atom(token.to_string()),
        })
    }
}

fn tokenize(source: &str) -> Vec<String> {
    lazy_static! {
        static ref RE:regex::Regex = Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###).unwrap();
    }

    let mut tokens: Vec<String> = Vec::new();
    for cap in RE.captures_iter(&source) {
        if cap[1].starts_with(";") || &cap[1] == "" {
            continue;
        }
        tokens.push(cap[1].to_string());
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
        assert_eq!(scan.scan().unwrap(), Token::LeftParen);
        assert_eq!(scan.scan().unwrap(), Token::Atom("+".to_string()));
        assert_eq!(scan.scan().unwrap(), Token::Atom("1".to_string()));
        assert_eq!(scan.scan().unwrap(), Token::Atom("2".to_string()));
        assert_eq!(scan.scan().unwrap(), Token::RightParen);
    }
}
