
#[derive(Clone, Debug, PartialEq)]
enum Ast {
    Bool(bool),
    Int(i64),
    Double(f64),
    List(Vec<Ast>),
    Str(String),
    Sym(String),
    Function(String, Vec<String>),
}

impl Ast {
    fn bool(b: bool) -> Ast {
        Ast::Bool(b)
    }

    fn int(i: i64) -> Ast {
        Ast::Int(i)
    }

    fn double(d: f64) -> Ast {
        Ast::Double(d)
    }

    fn list(l: Vec<Ast>) -> Ast {
        Ast::List(l)
    }

    fn str(s: &str) -> Ast {
        Ast::Str(s.to_string())
    }

    fn sym(s: &str) -> Ast {
        Ast::Str(s.to_string())
    }

    fn function(source: &str, params: Vec<&str>) -> Ast {
        Ast::Function(source.to_string(), params.iter().map(|x| {x.to_string()}).collect())
    }
}

mod test {
    use super::*;

    #[test]
    fn test_init_ast() {
        assert_eq!(Ast::bool(true), Ast::Bool(true));
        assert_eq!(Ast::int(1), Ast::Int(1));
        assert_eq!(Ast::double(1.0), Ast::Double(1.0));
        assert_eq!(Ast::list(vec![]), Ast::List(vec![]));
        assert_eq!(Ast::str("hi"), Ast::Str("hi".to_string()));
        assert_eq!(Ast::function("main", vec![]), Ast::Function("main".to_string(), vec![]));
    }

    #[test]
    fn test_list_recursion() {
        Ast::list(vec![Ast::int(1), Ast::int(2), Ast::int(3)]);
    }
}
