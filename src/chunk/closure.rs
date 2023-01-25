use crate::chunk::object::Function;

#[derive(Clone, Debug)]
pub struct ObjUpvalue {
    pub location: usize,
}

#[derive(Clone, Debug)]
pub struct Closure {
    pub function: Box<Function>,
    pub upvalues: Vec<ObjUpvalue>,
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
