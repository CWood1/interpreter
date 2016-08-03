use ast_executor::AstExecutor;

pub struct BinaryOperation {
    left: Box<AstExecutor>,
    right: Box<AstExecutor>,
    operation: BinaryOp
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl BinaryOperation {
    pub fn new(left: Box<AstExecutor>, operation: BinaryOp, right: Box<AstExecutor>) -> BinaryOperation {
        BinaryOperation {
            left: left,
            right: right,
            operation: operation,
        }
    }
}

impl AstExecutor for BinaryOperation {
    fn execute(&self) -> i32 {
        match self.operation {
            BinaryOp::Add => self.left.execute() + self.right.execute(),
            BinaryOp::Sub => self.left.execute() - self.right.execute(),
            BinaryOp::Mul => self.left.execute() * self.right.execute(),
            BinaryOp::Div => self.left.execute() / self.right.execute(),
            BinaryOp::Mod => self.left.execute() % self.right.execute(),
        }
    }
}
