use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Ast {
    Nil,
    Bool(bool),
    Int(i64),
    Double(f64),
    List(Vec<Ast>),
    Str(String),
    Sym(String),
    Function(String, Vec<String>),
    Block(String, Vec<Ast>),
}

impl Ast {
    pub fn nil() -> Ast {
        Ast::Nil
    }

    pub fn bool(b: bool) -> Ast {
        Ast::Bool(b)
    }

    pub fn int(i: i64) -> Ast {
        Ast::Int(i)
    }

    pub fn double(d: f64) -> Ast {
        Ast::Double(d)
    }

    pub fn list(l: Vec<Ast>) -> Ast {
        Ast::List(l)
    }

    pub fn str(s: &str) -> Ast {
        Ast::Str(s.to_string())
    }

    pub fn sym(s: &str) -> Ast {
        Ast::Sym(s.to_string())
    }

    pub fn function(source: &str, params: Vec<&str>) -> Ast {
        Ast::Function(
            source.to_string(),
            params.iter().map(|x| x.to_string()).collect(),
        )
    }

    pub fn block(name: &str, statements: &Vec<Ast>) -> Ast {
        Ast::Block(name.to_string(), statements.clone())
    }

    pub fn get_sym(&self) -> &String {
        match self {
            Ast::Sym(s) => s,
            _ => panic!(),
        }
    }

    pub fn get_list(&self) -> &Vec<Ast> {
        match self {
            Ast::List(ls) => ls,
            _ => panic!(),
        }
    }

    pub fn is_sym(&self) -> bool {
        match self {
            Ast::Sym(_) => true,
            _ => false,
        }
    }
}
