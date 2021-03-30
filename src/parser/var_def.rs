use super::expr::Expr;

#[derive(Debug, PartialEq)]
struct VarDef {
    name: String,
    value: Expr,
}

impl VarDef {
    fn new(s: &str) -> Self {
        let s = s.strip_prefix("let").unwrap();
        let s = s.trim_start();
        let first_whitespace_index = s.find(|c: char| c.is_whitespace()).unwrap();
        let (name, s) = s.split_at(first_whitespace_index);
        let s = s.trim_start().strip_prefix("=").unwrap().trim_start();
        let value = Expr::new(s);

        Self {
            name: name.to_string(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{number::Number, str::Str};
    use super::*;

    #[test]
    fn parse_var_def_number() {
        let var_def = VarDef::new("let max = 99");
        assert_eq!(
            var_def,
            VarDef {
                name: "max".to_string(),
                value: Expr::Number(Number { value: 99 }),
            },
        );
    }

    #[test]
    fn parse_var_def_str() {
        let var_def = VarDef::new("let name = \"ferris\"");
        assert_eq!(
            var_def,
            VarDef {
                name: "name".to_string(),
                value: Expr::Str(Str {
                    value: "ferris".to_string()
                }),
            },
        );
    }
}
