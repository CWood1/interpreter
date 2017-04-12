use ast_executor::AstExecutor;
use value::Value;

pub struct BinOpMul {
    left: Box<AstExecutor>,
    right: Box<AstExecutor>,
}

impl BinOpMul {
    pub fn new(left: Box<AstExecutor>, right: Box<AstExecutor>) -> BinOpMul {
        BinOpMul {
            left: left,
            right: right,
        }
    }
}

impl AstExecutor for BinOpMul {
    fn execute(&self) -> Value {
        let vals = (self.left.execute(), self.right.execute());

        match vals {
            (Value::Nu8(l), Value::Nu8(r)) => Value::Nu8(l * r),
            (Value::Ni8(l), Value::Ni8(r)) => Value::Ni8(l * r),
            _ => panic!("Type mismatch: attempted to add a u8 to an i8"),
        }
    }
}
