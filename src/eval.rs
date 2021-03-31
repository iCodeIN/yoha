use crate::parser::{expr::Expr, var_def::VarDef};
use std::collections::HashMap;

#[derive(Default)]
struct Env {
    vars: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
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

    fn eval_var_def(&mut self, var_def: VarDef) {
        self.vars
            .insert(var_def.name, self.eval_expr(var_def.value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::var::Var;

    #[test]
    fn eval_number() {
        let expr = Expr::new("92");
        let env = Env::default();
        assert_eq!(env.eval_expr(expr), Value::Number(92));
    }

    #[test]
    fn eval_str() {
        let expr = Expr::new("\"Hello, World!\"");
        let env = Env::default();
        assert_eq!(env.eval_expr(expr), Value::Str("Hello, World!".to_string()));
    }

    #[test]
    fn eval_var() {
        let mut env = Env::default();
        env.eval_var_def(VarDef::new("let foo = 92"));
        assert_eq!(env.eval_expr(Expr::Var(Var::new("foo"))), Value::Number(92));
    }

    #[test]
    fn eval_var_def() {
        let mut env = Env::default();
        env.eval_var_def(VarDef::new("let foo = 92"));
        env.eval_var_def(VarDef::new("let bar = foo"));
        env.eval_var_def(VarDef::new("let baz = bar"));
        assert_eq!(env.eval_expr(Expr::Var(Var::new("baz"))), Value::Number(92));
    }
}
