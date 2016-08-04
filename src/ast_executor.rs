use value::Value;

pub trait AstExecutor {
    fn execute(&self) -> Value;
}

impl AstExecutor for Vec<Box<AstExecutor>> {
    fn execute(&self) -> Value {
        for stmt in self {
            println!("{}", stmt.execute());
        }

        Value::None
    }
}
