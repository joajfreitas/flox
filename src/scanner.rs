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

//TODO: compare this with flang regex based implementation
fn tokenize(source: &str) -> Vec<(String, usize)> {
    let source = source.replace("(", " ( ").replace(")", " ) ");
    let lines = source.split('\n');

    lines
        .enumerate()
        .map(|(i, line)| {
            line.split_whitespace()
                .into_iter()
                .map(move |tok| (tok.to_string(), i + 1))
                .collect::<Vec<(String, usize)>>()
        })
        .flatten()
        .collect::<Vec<(String, usize)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut scanner = FloxScanner::new("");
        assert!(scanner.token().is_none());
    }

    #[test]
    fn test_tokenize() {
        let mut scan = FloxScanner::new("(+ 1 2)");
        assert_eq!(scan.token().unwrap().token(), &TokenType::LeftParen);
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("+".to_string())
        );
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("1".to_string())
        );
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("2".to_string())
        );
        assert_eq!(scan.token().unwrap().token(), &TokenType::RightParen);
    }

    #[test]
    fn test_tokenize_with_new_lines() {
        let mut scan = FloxScanner::new("(+ 1 2)\n(* 2 3)");
        assert_eq!(scan.token().unwrap().token(), &TokenType::LeftParen);
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("+".to_string())
        );
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("1".to_string())
        );
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("2".to_string())
        );
        assert_eq!(scan.token().unwrap().token(), &TokenType::RightParen);
        assert_eq!(scan.token().unwrap().token(), &TokenType::LeftParen);
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("*".to_string())
        );
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("2".to_string())
        );
        assert_eq!(
            scan.token().unwrap().token(),
            &TokenType::Atom("3".to_string())
        );
        assert_eq!(scan.token().unwrap().token(), &TokenType::RightParen);
    }

    #[test]
    fn test_scanner_iterator() {
        let scanner = FloxScanner::new("(+ 1 2)");
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
}
