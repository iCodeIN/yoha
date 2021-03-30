use crate::parser::expr::Expr;
use std::collections::HashMap;

struct Env {
    vars: HashMap<String, Value>,
}

#[derive(Clone)]
enum Value {
    Str(String),
    Number(i64),
}

impl Env {
    fn eval_expr(&self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n.value),
            Expr::Str(s) => Value::Str(s.value),
            Expr::Var(v) => self.vars.get(&v.name).unwrap().clone(),
        }
    }
}
