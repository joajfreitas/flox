use crate::scanner::Scanner;

#[derive(Debug, PartialEq)]
pub enum Ast {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    List(Vec<Ast>),
}

struct Parser {
    scanner: Scanner,
}

impl Parser {
    fn new(scanner: &Scanner) -> Parser {
        Parser {
            scanner: scanner.clone(),
        }
    }

    fn parse(&mut self) -> Result<Ast, String> {
        let token = match self.scanner.peek() {
            Some(x) => x,
            None => return Ok(Ast::Nil),
        };

        match &token.0 {
            Token::LeftParen => read_seq(scanner, chunk, compiler)?,
            Token::RightParen => return Err("unexpected ')'".to_string()),
            Token::Atom(_) => {
                scanner.scan().unwrap();
                read_atom(token, scanner, chunk, compiler);
            }
        }
        Ok(Ast::Int(1))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scanner::Scanner;

    #[test]
    fn test_create_ast_node() {
        assert_eq!(Ast::Int(32), Ast::Int(32));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Parser::new(&mut Scanner::new("1")).parse().unwrap(),
            Ast::Int(1)
        )
    }
}
