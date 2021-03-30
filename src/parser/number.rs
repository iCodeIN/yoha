#[derive(Debug, PartialEq)]
pub(crate) struct Number {
    pub(crate) value: i64,
}

impl Number {
    pub(crate) fn new(s: &str) -> Self {
        Self {
            value: s.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let number = Number::new("92");
        assert_eq!(Number { value: 92 }, number);
    }
}
