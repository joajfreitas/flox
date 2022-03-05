use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Sub};

use crate::chunk::closure::Closure;
use crate::chunk::object::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Obj(Box<Object>),
}

impl Value {
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Value::Number(f) => Some(*f),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get_str(&self) -> Option<&str> {
        match self {
            Value::Obj(obj) => obj.get_str(),
            _ => None,
        }
    }

    pub fn get_function(&self) -> Option<Box<Closure>> {
        match self {
            Value::Obj(obj) => obj.get_function(),
            _ => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_function(&self) -> bool {
        match self {
            Value::Obj(f) => f.is_function(),
            _ => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Number(b1), Value::Number(b2)) => b1 == b2,
            _ => panic!(),
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => Some(b1.cmp(b2)),
            (Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2),
            _ => panic!(),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (&self, &other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
            _ => {
                panic!();
            }
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),
            _ => panic!(),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
            _ => panic!(),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 / n2),
            _ => panic!(),
        }
    }
}

impl BitAnd for Value {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 & b2),
            _ => panic!(),
        }
    }
}

impl BitOr for Value {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 | b2),
            _ => panic!(),
        }
    }
}

impl BitXor for Value {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 ^ b2),
            _ => panic!(),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Value::Bool(!b),
            _ => panic!(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{:.1}", value),
            Value::Bool(value) => write!(f, "{:1}", value),
            Value::Nil => write!(f, "nil"),
            Value::Obj(obj) => match &**obj {
                Object::Str(s) => write!(f, "{:1}", s),
                Object::Function(closure) => write!(f, "{:?}", closure),
            },
        }
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
    fn test_value_get_number() {
        let number = Value::Number(1.0);
        assert_eq!(number.get_number(), Some(1.0));

        let boolean = Value::Bool(true);
        assert_eq!(boolean.get_number(), None)
    }

    #[test]
    fn test_value_get_bool() {
        let boolean = Value::Bool(true);
        assert_eq!(boolean.get_bool(), Some(true));

        let number = Value::Number(1.0);
        assert_eq!(number.get_bool(), None);
    }

    #[test]
    fn test_value_get_str() {
        let string = Value::Obj(Box::new(Object::Str("hello".to_string())));
        let boolean = Value::Bool(false);

        assert_eq!(string.get_str(), Some("hello"));
        assert_eq!(boolean.get_str(), None);
    }

    #[test]
    fn test_value_get_function() {
        let function = Value::Obj(Box::new(Object::Function(Box::new(fixture_closure()))));
        let boolean = Value::Bool(false);

        assert_eq!(function.get_function(), Some(Box::new(fixture_closure())));
        assert_eq!(boolean.get_function(), None);
    }

    #[test]
    fn test_value_is_nil() {
        let nil = Value::Nil;
        let boolean = Value::Bool(false);

        assert_eq!(nil.is_nil(), true);
        assert_eq!(boolean.is_nil(), false);
    }

    #[test]
    fn test_value_is_number() {
        let number = Value::Number(2.0);
        let boolean = Value::Bool(false);

        assert_eq!(number.is_number(), true);
        assert_eq!(boolean.is_number(), false);
    }

    #[test]
    fn test_value_is_bool() {
        let number = Value::Number(2.0);
        let boolean = Value::Bool(false);

        assert_eq!(boolean.is_bool(), true);
        assert_eq!(number.is_bool(), false);
    }

    #[test]
    fn test_value_is_function() {
        let function = Value::Obj(Box::new(Object::Function(Box::new(fixture_closure()))));
        let boolean = Value::Bool(false);

        assert_eq!(function.is_function(), true);
        assert_eq!(boolean.is_function(), false);
    }

    #[test]
    fn test_value_eq() {
        let True = Value::Bool(true);
        let False = Value::Bool(false);

        let number1 = Value::Number(2.0);
        let number2 = Value::Number(2.0);

        assert_eq!(number1, number2);
        assert_ne!(True, False);
    }

    #[test]
    #[should_panic]
    fn test_value_eq_panic() {
        Value::Nil == Value::Nil;
    }

    #[test]
    fn test_value_cmp() {
        let True = Value::Bool(true);
        let False = Value::Bool(false);

        let number1 = Value::Number(2.0);
        let number2 = Value::Number(2.1);

        assert_eq!(number1 < number2, true);
        assert_ne!(True < False, true);
    }

    #[test]
    #[should_panic]
    fn test_value_cmd_panic() {
        Value::Nil < Value::Nil;
    }

    #[test]
    fn test_value_add() {
        assert_eq!(Value::Number(3.0) + Value::Number(2.0), Value::Number(5.0));
    }

    #[test]
    #[should_panic]
    fn test_value_add_panic() {
        Value::Bool(true) + Value::Bool(false);
    }

    #[test]
    fn test_value_sub() {
        assert_eq!(Value::Number(3.0) - Value::Number(2.0), Value::Number(1.0));
    }

    #[test]
    #[should_panic]
    fn test_value_sub_panic() {
        Value::Bool(true) - Value::Bool(false);
    }

    #[test]
    fn test_value_mul() {
        assert_eq!(Value::Number(2.0) * Value::Number(3.0), Value::Number(6.0));
    }

    #[test]
    #[should_panic]
    fn test_value_mul_panic() {
        Value::Bool(true) * Value::Bool(false);
    }

    #[test]
    fn test_value_div() {
        assert_eq!(Value::Number(6.0) / Value::Number(2.0), Value::Number(3.0));
    }

    #[test]
    #[should_panic]
    fn test_value_div_panic() {
        Value::Bool(true) / Value::Bool(false);
    }

    #[test]
    fn test_value_and() {
        assert_eq!(Value::Bool(false) & Value::Bool(false), Value::Bool(false));
        assert_eq!(Value::Bool(false) & Value::Bool(true), Value::Bool(false));
        assert_eq!(Value::Bool(true) & Value::Bool(false), Value::Bool(false));
        assert_eq!(Value::Bool(true) & Value::Bool(true), Value::Bool(true));
    }

    #[test]
    #[should_panic]
    fn test_value_and_panic() {
        Value::Number(1.0) & Value::Number(2.0);
    }

    #[test]
    fn test_value_or() {
        assert_eq!(Value::Bool(false) | Value::Bool(false), Value::Bool(false));
        assert_eq!(Value::Bool(false) | Value::Bool(true), Value::Bool(true));
        assert_eq!(Value::Bool(true) | Value::Bool(false), Value::Bool(true));
        assert_eq!(Value::Bool(true) | Value::Bool(true), Value::Bool(true));
    }

    #[test]
    #[should_panic]
    fn test_value_or_panic() {
        Value::Number(1.0) | Value::Number(2.0);
    }

    #[test]
    fn test_value_xor() {
        assert_eq!(Value::Bool(false) ^ Value::Bool(false), Value::Bool(false));
        assert_eq!(Value::Bool(false) ^ Value::Bool(true), Value::Bool(true));
        assert_eq!(Value::Bool(true) ^ Value::Bool(false), Value::Bool(true));
        assert_eq!(Value::Bool(true) ^ Value::Bool(true), Value::Bool(false));
    }

    #[test]
    #[should_panic]
    fn test_value_xor_panic() {
        Value::Number(1.0) ^ Value::Number(2.0);
    }

    #[test]
    fn test_value_not() {
        assert_eq!(!Value::Bool(false), Value::Bool(true));
        assert_eq!(!Value::Bool(true), Value::Bool(false));
    }

    #[test]
    #[should_panic]
    fn test_value_not_panic() {
        !Value::Number(1.0);
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Number(1.0)), "1.0");
        assert_eq!(format!("{}", Value::Number(-1.0)), "-1.0");
        assert_eq!(format!("{}", Value::Bool(false)), "false");
        assert_eq!(format!("{}", Value::Bool(true)), "true");
        assert_eq!(format!("{}", Value::Nil), "nil");
        assert_eq!(
            format!("{}", Value::Obj(Box::new(Object::Str("ola".to_string())))),
            "ola"
        );
        assert_eq!(
            format!(
                "{}",
                Value::Obj(Box::new(Object::Function(Box::new(fixture_closure()))))
            ),
            "(test_closure (x y))"
        );
    }
}
