use super::number::Number;

#[derive(Debug, PartialEq)]
struct VarDef {
    name: String,
    value: Number,
}

impl VarDef {
    fn new(s: &str) -> Self {
        let s = s.strip_prefix("let").unwrap();
        let s = s.trim_start();
        let first_whitespace_index = s.find(|c: char| c.is_whitespace()).unwrap();
        let (name, s) = s.split_at(first_whitespace_index);
        let s = s.trim_start().strip_prefix("=").unwrap().trim_start();
        let number = Number::new(s);

        Self {
            name: name.to_string(),
            value: number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_var_def() {
        let var_def = VarDef::new("let max = 99");
        assert_eq!(
            var_def,
            VarDef {
                name: "max".to_string(),
                value: Number { value: 99 },
            },
        );
    }
}
