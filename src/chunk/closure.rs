use crate::chunk::object::Function;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjUpvalue {
    pub location: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Closure {
    pub function: Box<Function>,
    pub upvalues: Vec<ObjUpvalue>,
}

impl fmt::Debug for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(fn {})", self.function.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;

    fn fixture_closure() -> Closure {
        Closure {
            function: Box::new(Function {
                arity: 2,
                chunk: Chunk::new("test_chunk"),
                name: "test_closure".to_string(),
                upvalue_count: 0,
            }),
            upvalues: vec![],
        }
    }

    #[test]
    fn test_chunk_debug() {
        let closure = fixture_closure();
        let result = format!("{:?}", closure);
        assert_eq!(result, "Closure { function: (test_closure), upvalues: [] }");
    }
}
