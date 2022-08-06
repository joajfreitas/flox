use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Atom(String),
}

impl TokenType {
    pub fn atom(s: &str) -> TokenType {
        TokenType::Atom(s.to_string())
    }

    pub fn get_atom(&self) -> &String {
        match self {
            TokenType::Atom(atom) => atom,
            _ => panic!(),
        }
    }
}

pub trait Token {
    fn line(&self) -> usize;
    fn token(&self) -> &TokenType;
    fn get(&self) -> (&TokenType, usize);
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloxToken {
    line: usize,
    token: TokenType,
}

impl FloxToken {
    pub fn new(token: TokenType, line: usize) -> FloxToken {
        FloxToken { token, line }
    }

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

    fn get(&self) -> (&TokenType, usize) {
        (self.token(), self.line())
    }
}

pub trait Scanner: Iterator {
    fn token(&mut self) -> Option<Box<dyn Token>>;
    fn peek(&mut self) -> Option<Box<dyn Token>>;
    fn feed(&mut self, source: &str);
}

#[derive(Debug, Clone)]
pub struct FloxScanner {
    tokens: Vec<(String, usize)>,
    pos: usize,
}

impl FloxScanner {
    pub fn new(source: &str) -> FloxScanner {
        FloxScanner {
            tokens: tokenize(source),
            pos: 0,
        }
    }
}

impl Iterator for FloxScanner {
    type Item = Box<dyn Token>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.token()?)
    }
}

impl Scanner for FloxScanner {
    fn token(&mut self) -> Option<Box<dyn Token>> {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn peek(&mut self) -> Option<Box<dyn Token>> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let (token, line) = &self.tokens[self.pos];
        let token = match &token as &str {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            _ => TokenType::Atom(token.to_string()),
        };

        Some(Box::new(FloxToken { line: *line, token }))
    }

    fn feed(&mut self, source: &str) {
        self.tokens = tokenize(source);
        self.pos = 0;
    }
}

fn tokenize(source: &str) -> Vec<(String, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
        )
        .unwrap();
    }

    let mut tokens: Vec<(String, usize)> = Vec::new();

    for (line_number, line) in source.split("\n").into_iter().enumerate() {
        for cap in RE.captures_iter(&line) {
            if cap[1].starts_with(";") {
                continue;
            }
            if &cap[1] != "" {
                tokens.push((cap[1].to_string(), line_number + 1));
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn scanner() -> FloxScanner {
        FloxScanner::new("")
    }

    #[rstest]
    fn test_empty(mut scanner: FloxScanner) {
        assert!(scanner.token().is_none());
    }

    #[rstest]
    fn test_tokenize(mut scanner: FloxScanner) {
        scanner.feed("(+ 1 2)");
        assert_eq!(scanner.token().unwrap().get(), (&TokenType::LeftParen, 1));
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("+".to_string()), 1)
        );
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("1".to_string()), 1)
        );
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("2".to_string()), 1)
        );
        assert_eq!(scanner.token().unwrap().get(), (&TokenType::RightParen, 1));
    }

    #[rstest]
    fn test_tokenize_with_new_lines(mut scanner: FloxScanner) {
        scanner.feed("(+ 1 2)\n(* 2 3)");

        assert_eq!(scanner.token().unwrap().token(), &TokenType::LeftParen);
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("+".to_string()), 1)
        );
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("1".to_string()), 1)
        );
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("2".to_string()), 1)
        );
        assert_eq!(scanner.token().unwrap().get(), (&TokenType::RightParen, 1));
        assert_eq!(scanner.token().unwrap().get(), (&TokenType::LeftParen, 2));
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("*".to_string()), 2)
        );
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("2".to_string()), 2)
        );
        assert_eq!(
            scanner.token().unwrap().get(),
            (&TokenType::Atom("3".to_string()), 2)
        );
        assert_eq!(scanner.token().unwrap().get(), (&TokenType::RightParen, 2));
    }

    #[rstest]
    fn test_scanner_iterator(mut scanner: FloxScanner) {
        scanner.feed("(+ 1 2)");
        let tokens = vec![
            TokenType::LeftParen,
            TokenType::atom("+"),
            TokenType::atom("1"),
            TokenType::atom("2"),
            TokenType::RightParen,
        ];

        for (i, token) in scanner.clone().into_iter().enumerate() {
            assert_eq!(token.token(), &tokens[i]);
        }
    }

    #[rstest]
    fn test_scanner_string(mut scanner: FloxScanner) {
        scanner.feed("\"hello world\"");
        assert_eq!(
            *scanner.token().unwrap().token(),
            TokenType::Atom("\"hello world\"".to_string())
        );
    }
}
