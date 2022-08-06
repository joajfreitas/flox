/* Convert the vector of ast nodes:
 *      into a block in case of multiple nodes
 *      into the single node in case there is only one
 */

use super::ast::Ast;
use super::parser::L1Parser;
use crate::scanner::{Scanner, Token, TokenType};

struct FloxL1Parser {
    ast: Ast,
}

impl FloxL1Parser {
    fn new() -> FloxL1Parser {
        FloxL1Parser { ast: Ast::Nil }
    }
}

impl L1Parser for FloxL1Parser {
    fn parse(&mut self, input: Vec<Ast>) -> Ast {
        if input.len() == 0 {
            Ast::Nil
        } else if input.len() == 1 {
            input[0].clone()
        } else {
            Ast::block("placeholder", &input)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[fixture]
    fn l1_parser() -> FloxL1Parser {
        FloxL1Parser::new()
    }

    #[rstest]
    fn test_init(mut l1_parser: FloxL1Parser) {
        assert_eq!(l1_parser.parse(vec![]), Ast::Nil);
    }

    #[rstest]
    fn test_single_element(mut l1_parser: FloxL1Parser) {
        assert_eq!(l1_parser.parse(vec![Ast::Nil]), Ast::Nil);
        assert_eq!(l1_parser.parse(vec![Ast::Bool(true)]), Ast::Bool(true));
        assert_eq!(l1_parser.parse(vec![Ast::Int(1)]), Ast::Int(1));
        assert_eq!(l1_parser.parse(vec![Ast::Double(2.0)]), Ast::Double(2.0));
        assert_eq!(
            l1_parser.parse(vec![Ast::List(vec![Ast::Int(2)])]),
            Ast::List(vec![Ast::Int(2)])
        );
        assert_eq!(l1_parser.parse(vec![Ast::str("hello")]), Ast::str("hello"));
        assert_eq!(l1_parser.parse(vec![Ast::sym("hello")]), Ast::sym("hello"));
    }

    #[rstest]
    fn test_multiple_element(mut l1_parser: FloxL1Parser) {
        assert_eq!(
            l1_parser.parse(vec![
                Ast::List(vec![Ast::sym("*"), Ast::Int(1), Ast::Int(2)]),
                Ast::List(vec![Ast::sym("+"), Ast::Int(1), Ast::Int(1)])
            ]),
            Ast::block(
                "placeholder",
                &vec![
                    Ast::List(vec![Ast::sym("*"), Ast::Int(1), Ast::Int(2)]),
                    Ast::List(vec![Ast::sym("+"), Ast::Int(1), Ast::Int(1)])
                ]
            )
        )
    }
}
