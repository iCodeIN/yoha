use super::{number::Number, str::Str};

#[derive(Debug, PartialEq)]
enum Expr {
    Number(Number),
    Str(Str),
}

impl Expr {
    fn new(s: &str) -> Self {
        if s.starts_with('\"') {
            Self::Str(Str::new(s))
        } else {
            Self::Number(Number::new(s))
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
}
