use core::panic;

#[derive(Debug, PartialEq)]
struct Str {
    value: String,
}

impl Str {
    fn new(s: &str) -> Self {
        if s.starts_with('\"') && s.ends_with('\"') {
            Self {
                value: s[1..s.len() - 1].to_string(),
            }
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str() {
        let str = Str::new("\"ninety two\"");
        assert_eq!(
            Str {
                value: "ninety two".to_string(),
            },
            str
        );
    }
}
