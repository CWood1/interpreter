use ast_executor::AstExecutor;
use value::Value;

pub struct BinOpSub {
    left: Box<AstExecutor>,
    right: Box<AstExecutor>,
}

impl BinOpSub {
    pub fn new(left: Box<AstExecutor>, right: Box<AstExecutor>) -> BinOpSub {
        BinOpSub {
            left: left,
            right: right,
        }
    }
}

impl AstExecutor for BinOpSub {
    fn execute(&self) -> Value {
        let vals = (self.left.execute(), self.right.execute());

        match vals {
            (Value::Nu8(l), Value::Nu8(r)) => Value::Nu8(l - r),
            (Value::Ni8(l), Value::Ni8(r)) => Value::Ni8(l - r),
            _ => panic!("Type mismatch: attempted to subtract a u8 and an i8"),
        }
    }
}
