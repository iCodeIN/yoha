use super::{number::Number, str::Str, var::Var};

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Number(Number),
    Str(Str),
    Var(Var),
}

impl Expr {
    pub(crate) fn new(s: &str) -> Self {
        if s.starts_with('\"') {
            Self::Str(Str::new(s))
        } else if s.starts_with(|c: char| c.is_ascii_digit()) {
            Self::Number(Number::new(s))
        } else {
            Self::Var(Var::new(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let expr = Expr::new("92");
        assert_eq!(expr, Expr::Number(Number { value: 92 }));
    }

    #[test]
    fn parse_str() {
        let expr = Expr::new("\"ninety two\"");
        assert_eq!(
            expr,
            Expr::Str(Str {
                value: "ninety two".to_string()
            })
        );
    }

    #[test]
    fn parse_var() {
        let expr = Expr::new("foo");
        assert_eq!(
            expr,
            Expr::Var(Var {
                name: "foo".to_string()
            })
        );
    }
}
