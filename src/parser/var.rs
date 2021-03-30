#[derive(Debug, PartialEq)]
struct Var {
    name: String,
}

impl Var {
    fn new(s: &str) -> Self {
        Self {
            name: s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_var() {
        assert_eq!(
            Var::new("foo"),
            Var {
                name: "foo".to_string()
            }
        );
    }
}
