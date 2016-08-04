use ast_executor::AstExecutor;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Nu8(u8),
    Ni8(i8),
    Nu16(u16),
    Ni16(i16),
    Nu32(u32),
    Ni32(i32),
    None,
}

impl AstExecutor for Value {
    fn execute(&self) -> Value {
        self.clone()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Nu8(n) => write!(f, "{}", n),
            &Value::Ni8(n) => write!(f, "{}", n),
            &Value::Nu16(n) => write!(f, "{}", n),
            &Value::Ni16(n) => write!(f, "{}", n),
            &Value::Nu32(n) => write!(f, "{}", n),
            &Value::Ni32(n) => write!(f, "{}", n),
            &Value::None => write!(f, ""),
        }
    }
}
