/* Apply function transformations
 * Assumes that any list that starts with a symbol is a function
 */

use super::ast::Ast;
use super::parser::L2Parser;
use crate::scanner::{Scanner, Token, TokenType};

struct FloxL2Parser {
    ast: Ast,
}

impl FloxL2Parser {
    fn new() -> FloxL2Parser {
        FloxL2Parser { ast: Ast::Nil }
    }
}

impl L2Parser for FloxL2Parser {
    fn parse(&mut self, input: Ast) -> Ast {
        match input {
            Ast::List(nodes) => {
                if nodes[0].is_sym() {
                    Ast::Function("")
                }
            }
            _ => input,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[fixture]
    fn l2_parser() -> FloxL2Parser {
        FloxL2Parser::new()
    }

    #[rstest]
    fn test_init(mut l2_parser: FloxL2Parser) {
        assert_eq!(l2_parser.parse(Ast::Nil), Ast::Nil);
    }
}
