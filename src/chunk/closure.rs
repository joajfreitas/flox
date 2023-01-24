use std::fmt;

use crate::chunk::value::Value;
use crate::chunk::Chunk;
use crate::compiler::UpValue;

//#[derive(Debug, Clone)]
//pub struct Upvalue {
//    pub location: Value,
//}

#[derive(Clone)]
pub struct Closure {
    pub params: Vec<String>,
    pub chunk: Chunk,
    pub name: String,
    pub upvalues: Vec<UpValue>,
}

impl fmt::Debug for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} (", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i + 1 == self.params.len() {
                write!(f, "{}", param)?;
            } else {
                write!(f, "{} ", param)?;
            }
        }

        write!(f, "))")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_closure() -> Closure {
        Closure {
            params: vec!["x".to_string(), "y".to_string()],
            chunk: Chunk::new("test_chunk"),
            name: "test_closure".to_string(),
        }
    }

    #[test]
    fn test_chunk_debug() {
        let closure = fixture_closure();
        let result = format!("{:?}", closure);
        assert_eq!(result, "(test_closure (x y))")
    }
}
