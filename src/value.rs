use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(Rc<str>),
}

impl From<f64> for Value {
    fn from(number: f64) -> Value {
        Self::Number(number)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Value {
        Self::String(Rc::from(string))
    }
}
