enum Ast {
    Int(i64),
    Double(f64),
    List(Vec<Ast>),
    Str(String),
    Function(String, Vec<String>),
}

impl Ast {
    fn int(i: i64) -> Ast {
        Ast::Int(i)
    }

    fn double(d: f64) -> Ast {
        Ast::Double(d)
    }

    fn list(l: Vec<Ast>) -> Ast {
        Ast::List(l)
    }

    fn str(s: String) -> Ast {
        Ast::Str(s)
    }

    fn function(source: String, params: Vec<String>) -> Ast {
        Ast::Function(source, params)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_init_ast() {}
}
