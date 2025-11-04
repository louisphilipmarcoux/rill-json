#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
}

fn parse_null(input: &str) -> Result<(JsonValue, &str), &'static str> {
    if input.starts_with("null") {
        Ok((JsonValue::Null, &input[4..]))
    } else {
        Err("Expected 'null'")
    }
}

fn parse_boolean(input: &str) -> Result<(JsonValue, &str), &'static str> {
    if input.starts_with("true") {
        Ok((JsonValue::Boolean(true), &input[4..]))
    } else if input.starts_with("false") {
        Ok((JsonValue::Boolean(false), &input[5..]))
    } else {
        Err("Expected 'true' or 'false'")
    }
}

fn parse_number(input: &str) -> Result<(JsonValue, &str), &'static str> {
    let end_index = input
        .find(|c: char| c.is_whitespace() || c == ',' || c == ']' || c == '}')
        .unwrap_or(input.len());
    let num_str = &input[..end_index];
    match num_str.parse::<f64>() {
        Ok(num) => Ok((JsonValue::Number(num), &input[end_index..])),
        Err(_) => Err("Invalid number"),
    }
}

fn parse_value(input: &str) -> Result<(JsonValue, &str), &'static str> {
    match input.chars().next() {
        Some('n') => parse_null(input),
        Some('t') | Some('f') => parse_boolean(input),
        Some('-') | Some('0'..='9') => parse_number(input),
        _ => Err("Unexpected 'null', 'true', or 'false'"),
    }
}

fn main() {
    println!("JSON Parser: Run 'cargo test' to execute tests.");
}

#[cfg(test)]
mod tests {
    // Import everything from our main module
    use super::*;

    // --- Stage 1 Tests ---
    #[test]
    fn test_parse_null() {
        let (value, rest) = parse_value("null").unwrap();
        assert_eq!(value, JsonValue::Null);
        assert_eq!(rest, "");

        let (value, rest) = parse_value("null, 123").unwrap();
        assert_eq!(value, JsonValue::Null);
        assert_eq!(rest, ", 123");

        assert!(parse_value("nul").is_err());
        assert!(parse_value("NULL").is_err());
        assert!(parse_value("\"null\"").is_err());
    }

    // --- Stage 2 Tests ---
    #[test]
    fn test_parse_booleans() {
        let (value, rest) = parse_value("true").unwrap();
        assert_eq!(value, JsonValue::Boolean(true));
        assert_eq!(rest, "");

        let (value, rest) = parse_value("false").unwrap();
        assert_eq!(value, JsonValue::Boolean(false));
        assert_eq!(rest, "");

        let (value, rest) = parse_value("true}").unwrap();
        assert_eq!(value, JsonValue::Boolean(true));
        assert_eq!(rest, "}");

        assert!(parse_value("True").is_err());
        assert!(parse_value("fals").is_err());
        assert!(parse_value("ttrue").is_err());
    }

    // --- Stage 3 Tests ---
    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_value("0").unwrap(), (JsonValue::Number(0.0), ""));
        assert_eq!(parse_value("123").unwrap(), (JsonValue::Number(123.0), ""));
        assert_eq!(
            parse_value("-123").unwrap(),
            (JsonValue::Number(-123.0), "")
        );

        assert_eq!(
            parse_value("45.67").unwrap(),
            (JsonValue::Number(45.67), "")
        );
        assert_eq!(parse_value("-0.5").unwrap(), (JsonValue::Number(-0.5), ""));

        let (value, rest) = parse_value("123, 456").unwrap();
        assert_eq!(value, JsonValue::Number(123.0));
        assert_eq!(rest, ", 456");

        let (value, rest) = parse_value("-1.2]").unwrap();
        assert_eq!(value, JsonValue::Number(-1.2));
        assert_eq!(rest, "]");

        assert!(parse_value("1.2.3").is_err());
        assert!(parse_value("1-2").is_err());
        assert!(parse_value("--1").is_err());
        assert!(parse_value("abc").is_err());
    }
}
