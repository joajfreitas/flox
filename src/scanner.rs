#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LeftParen,
    RightParen,
    Atom(String),
}

impl Token {
    pub fn atom(&self) -> Result<String, String> {
        match self {
            Token::Atom(s) => Ok(s.clone()),
            _ => Err(format!("Expected atom, got: {:?}", self)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scanner {
    tokens: Vec<(String, usize)>,
    pos: usize,
    current_line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            tokens: tokenize(source),
            pos: 0,
            current_line: 0,
        }
    }

    pub fn scan(&mut self) -> Option<(Token, usize)> {
        let token = self.peek();
        self.pos += 1;
        token
    }

    pub fn peek(&mut self) -> Option<(Token, usize)> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let (token, line) = &self.tokens[self.pos];
        let token = match token as &str {
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            _ => Token::Atom(token.to_string()),
        };
        self.current_line = *line;
        Some((token, *line))
    }

    pub fn previous(&self) -> Option<Token> {
        let token: &str = &self.tokens[self.pos - 1].0;
        match token {
            "(" => Some(Token::LeftParen),
            ")" => Some(Token::RightParen),
            _ => Some(Token::Atom(token.to_string())),
        }
    }

    pub fn next_tokens(&self) -> Vec<String> {
        self.tokens[self.pos..]
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<String>>()
    }

    pub fn get_line(&self) -> usize {
        self.current_line
    }
}

fn tokenize(source: &str) -> Vec<(String, usize)> {
    let source = source.replace('(', " ( ");
    let source = source.replace(')', " ) ");
    let lines = source.split('\n');
    let mut tokens: Vec<(String, usize)> = Vec::new();

    for (i, line) in lines.enumerate() {
        for tok in line.split_whitespace() {
            tokens.push((tok.to_string(), i));
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
        assert_eq!(scan.scan().unwrap().0, Token::LeftParen);
        assert_eq!(scan.scan().unwrap().0, Token::Atom("+".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::Atom("1".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::Atom("2".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::RightParen);
    }

    #[test]
    fn test_tokenize_with_new_lines() {
        let mut scan = Scanner::new("(+ 1 2)\n(* 2 3)");
        assert_eq!(scan.scan().unwrap().0, Token::LeftParen);
        assert_eq!(scan.scan().unwrap().0, Token::Atom("+".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::Atom("1".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::Atom("2".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::RightParen);
        assert_eq!(scan.scan().unwrap().0, Token::LeftParen);
        assert_eq!(scan.scan().unwrap().0, Token::Atom("*".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::Atom("2".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::Atom("3".to_string()));
        assert_eq!(scan.scan().unwrap().0, Token::RightParen);
    }
}
