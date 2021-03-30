#[derive(Debug, PartialEq)]
struct Str {
    value: String,
}

impl Str {
    fn new(s: &str) -> Result<Self, &'static str> {
        if s.starts_with('\"') && s.ends_with('\"') {
            Ok(Self {
                value: s[1..s.len() - 1].to_string(),
            })
        } else {
            Err("failed to parse string")
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
            str,
            Ok(Str {
                value: "ninety two".to_string(),
            }),
        );
    }

    #[test]
    fn fail_to_parse_str() {
        assert_eq!(Str::new("🦀"), Err("failed to parse string"));
    }
}
