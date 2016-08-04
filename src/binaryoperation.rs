use ast_executor::AstExecutor;
use value::Value;

pub struct BinOpAdd {
    left: Box<AstExecutor>,
    right: Box<AstExecutor>,
}

impl BinOpAdd {
    pub fn new(left: Box<AstExecutor>, right: Box<AstExecutor>) -> BinOpAdd {
        BinOpAdd {
            left: left,
            right: right,
        }
    }
}

impl AstExecutor for BinOpAdd {
    fn execute(&self) -> Value {
        let vals = (self.left.execute(), self.right.execute());

        match vals {
            (Value::Nu8(l), Value::Nu8(r)) => Value::Nu8(l + r),
            (Value::Ni8(l), Value::Ni8(r)) => Value::Ni8(l + r),
            _ => panic!("Type mismatch: attempted to add a u8 to an i8"),
        }
    }
}
