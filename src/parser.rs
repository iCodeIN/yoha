#[derive(Debug, PartialEq)]
struct Number {
    value: i64,
}

impl Number {
    fn new(s: &str) -> Result<Self, &'static str> {
        Ok(Number {
            value: s.parse().map_err(|_| "failed to parse number")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let number = Number::new("92");
        assert_eq!(number, Ok(Number { value: 92 }));
    }

    #[test]
    fn fail_to_parse_number() {
        assert_eq!(Number::new("ninety two"), Err("failed to parse number"));
    }
}
