use core::panic;

#[derive(Debug, PartialEq)]
pub(crate) struct Str {
    pub(crate) value: String,
}

impl Str {
    pub(crate) fn new(s: &str) -> Self {
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
            str,
            Str {
                value: "ninety two".to_string(),
            }
        );
    }
}
