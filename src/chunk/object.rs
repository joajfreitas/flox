use crate::chunk::closure::Closure;
use crate::chunk::Chunk;
use std::fmt;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub chunk: Chunk,
    pub upvalue_count: usize,
    pub arity: usize,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Function(Box<Function>),
    Closure(Box<Closure>),
}

impl Object {
    pub fn get_str(&self) -> Option<&str> {
        match self {
            Object::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_function(&self) -> Option<Box<Function>> {
        match self {
            Object::Function(f) => Some(f.clone()),
            _ => None,
        }
    }

    pub fn get_closure(&self) -> Option<Box<Closure>> {
        match self {
            Object::Closure(f) => Some(f.clone()),
            _ => None,
        }
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Object::Function(_))
    }

    pub fn is_closure(&self) -> bool {
        matches!(self, Object::Closure(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;

    fn fixture_closure() -> Closure {
        Closure {
            params: vec!["x".to_string(), "y".to_string()],
            chunk: Chunk::new("test_chunk"),
            name: "test_closure".to_string(),
        }
    }

    #[test]
    fn test_object_get_str_with_value() {
        let object = Object::Str("some string".to_string());

        assert_eq!(object.get_str(), Some("some string"))
    }

    #[test]
    fn test_object_get_str_without_value() {
        let object = Object::Function(Box::new(fixture_closure()));

        assert_eq!(object.get_str(), None)
    }

    #[test]
    fn test_object_get_function_with_value() -> Result<(), String> {
        let object = Object::Function(Box::new(fixture_closure()));

        let function = object.get_function().ok_or("Failed to find function")?;
        assert_eq!(function.name, "test_closure");
        assert_eq!(function.params, vec!["x", "y"]);
        Ok(())
    }

    #[test]
    fn test_object_get_function_without_value() {
        let object = Object::Str("some string".to_string());

        assert_eq!(object.get_function(), None)
    }

    #[test]
    fn test_object_is_function() {
        let function = Object::Function(Box::new(fixture_closure()));
        let string = Object::Str("some string".to_string());

        assert_eq!(function.is_function(), true);
        assert_eq!(string.is_function(), false);
    }
}
