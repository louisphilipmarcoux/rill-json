use std::collections::HashMap;

// --- 1. JsonValue Enum ---
#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

// --- 2. Parser Functions ---

fn skip_whitespace(input: &str) -> &str {
    input.trim_start()
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
        Err(_) => Err("Invalid number format"),
    }
}

// --- UPDATED FUNCTION for Stage 8 ---
/// Tries to parse a JSON string, handling escape sequences.
fn parse_string(input: &str) -> Result<(JsonValue, &str), &'static str> {
    if !input.starts_with('"') {
        return Err("Expected '\"' at start of string");
    }

    // We'll build the new string content here
    let mut parsed_content = String::new();
    // We need an iterator of chars to look ahead
    let mut chars = input[1..].chars().enumerate();

    while let Some((i, c)) = chars.next() {
        match c {
            '\\' => {
                // Escape sequence: look at the next character
                if let Some((_, escaped_char)) = chars.next() {
                    match escaped_char {
                        '"' => parsed_content.push('"'),
                        '\\' => parsed_content.push('\\'),
                        '/' => parsed_content.push('/'), // Often escaped, though not required
                        'b' => parsed_content.push('\u{0008}'), // Backspace
                        'f' => parsed_content.push('\u{000C}'), // Form feed
                        'n' => parsed_content.push('\n'), // Newline
                        'r' => parsed_content.push('\r'), // Carriage return
                        't' => parsed_content.push('\t'), // Tab
                        // Stage 9 will handle 'u'
                        _ => return Err("Invalid escape sequence"), // e.g., \a, \z [cite: 150]
                    }
                } else {
                    // Reached end of input after a backslash
                    return Err("Unmatched '\"' at end of string");
                }
            }
            '"' => {
                // End of the string
                // The rest of the input starts *after* this closing quote
                // `i` is the index of the quote *within* &input[1..],
                // so the slice end is `i + 2` relative to the original `input`.
                let rest = &input[i + 2..];
                return Ok((JsonValue::String(parsed_content), rest));
            }
            // A regular character
            _ => parsed_content.push(c),
        }
    }

    // If we get here, the loop finished without finding a closing "
    Err("Unmatched '\"' at end of string")
}
// --- END UPDATED FUNCTION ---

fn parse_array(input: &str) -> Result<(JsonValue, &str), &'static str> {
    if !input.starts_with('[') {
        return Err("Expected '[' at start of array");
    }
    let mut current_input = skip_whitespace(&input[1..]);
    let mut elements = Vec::new();
    if current_input.starts_with(']') {
        return Ok((JsonValue::Array(elements), &current_input[1..]));
    }
    loop {
        let (value, rest) = parse_value(current_input)?;
        elements.push(value);
        current_input = skip_whitespace(rest);
        if current_input.starts_with(',') {
            current_input = skip_whitespace(&current_input[1..]);
        } else if current_input.starts_with(']') {
            current_input = &current_input[1..];
            break;
        } else {
            return Err("Expected ',' or ']' after array element");
        }
    }
    Ok((JsonValue::Array(elements), current_input))
}

fn parse_object(input: &str) -> Result<(JsonValue, &str), &'static str> {
    if !input.starts_with('{') {
        return Err("Expected '{' at start of object");
    }
    let mut current_input = skip_whitespace(&input[1..]);
    let mut map = HashMap::new();
    if current_input.starts_with('}') {
        return Ok((JsonValue::Object(map), &current_input[1..]));
    }
    loop {
        let (key_value, rest) = parse_string(current_input)?;
        let key = match key_value {
            JsonValue::String(s) => s,
            _ => return Err("Object key is not a string"),
        };
        current_input = skip_whitespace(rest);
        if !current_input.starts_with(':') {
            return Err("Expected ':' after object key");
        }
        current_input = skip_whitespace(&current_input[1..]);
        let (value, rest) = parse_value(current_input)?;
        map.insert(key, value);
        current_input = skip_whitespace(rest);
        if current_input.starts_with(',') {
            current_input = skip_whitespace(&current_input[1..]);
        } else if current_input.starts_with('}') {
            current_input = &current_input[1..];
            break;
        } else {
            return Err("Expected ',' or '}' after object value");
        }
    }
    Ok((JsonValue::Object(map), current_input))
}

/// Tries to parse any valid JSON value from the beginning of the input.
fn parse_value(input: &str) -> Result<(JsonValue, &str), &'static str> {
    let input = skip_whitespace(input);
    let parse_result = match input.chars().next() {
        Some('n') => parse_null(input),
        Some('t') | Some('f') => parse_boolean(input),
        Some('-') | Some('0'..='9') => parse_number(input),
        Some('"') => parse_string(input),
        Some('[') => parse_array(input),
        Some('{') => parse_object(input),
        Some(_) => Err("Invalid character at start of value"),
        None => Err("Unexpected end of input"),
    };
    parse_result.map(|(value, rest)| (value, skip_whitespace(rest)))
}

// --- 3. Main Function ---
fn main() {
    println!("JSON Parser. Run 'cargo test' to execute tests.");
}

// --- 4. Test Module ---
#[cfg(test)]
mod tests {
    use super::*;

    // Helper macro for object tests
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut map = HashMap::new();
                $(
                    map.insert($key.to_string(), $value);
                )*
                map
            }
        };
    }

    #[test]
    fn test_parse_null() { /* ... (keep your old tests) ... */
    }
    #[test]
    fn test_parse_booleans() { /* ... (keep your old tests) ... */
    }
    #[test]
    fn test_parse_numbers() { /* ... (keep your old tests) ... */
    }

    // --- UPDATED to split basic and escape tests ---
    #[test]
    fn test_parse_strings_basic() {
        // Valid empty string
        assert_eq!(
            parse_value("\"\"").unwrap(),
            (JsonValue::String("".to_string()), "")
        );
        // Valid simple string
        assert_eq!(
            parse_value("\"hello\"").unwrap(),
            (JsonValue::String("hello".to_string()), "")
        );
        // Valid with trailing data
        assert_eq!(
            parse_value("\"hello\", 123").unwrap(),
            (JsonValue::String("hello".to_string()), ", 123")
        );
        // Invalid: Unmatched quote
        assert!(parse_value("\"hello").is_err());
        // Invalid: Unquoted string
        assert!(parse_value("hello").is_err());
    }

    #[test]
    fn test_parse_arrays() { /* ... (keep your old tests) ... */
    }
    #[test]
    fn test_parse_objects() { /* ... (keep your old tests) ... */
    }
    #[test]
    fn test_parse_with_whitespace() { /* ... (keep your old tests) ... */
    }

    // --- NEW TESTS for Stage 8 ---
    #[test]
    fn test_parse_string_escapes() {
        // Test escaped quote [cite: 141]
        let (value, _) = parse_value("\"hello \\\"quoted\\\" world\"").unwrap();
        assert_eq!(
            value,
            JsonValue::String("hello \"quoted\" world".to_string())
        );

        // Test escaped backslash [cite: 145]
        let (value, _) = parse_value("\"\\\\\"").unwrap();
        assert_eq!(value, JsonValue::String("\\".to_string()));

        // Test common escapes [cite: 143]
        let (value, _) = parse_value("\"line1\\nline2\\t-tabbed\"").unwrap();
        assert_eq!(
            value,
            JsonValue::String("line1\nline2\t-tabbed".to_string())
        );

        // Test all valid simple escapes
        let (value, _) = parse_value("\"\\\"\\\\\\/\\b\\f\\n\\r\\t\"").unwrap();
        assert_eq!(
            value,
            JsonValue::String("\"\\/\u{0008}\u{000C}\n\r\t".to_string())
        );

        // Invalid: Invalid escape sequence [cite: 149, 150]
        assert!(parse_value("\"hello \\ world\"").is_err());
        assert!(parse_value("\"invalid \\a escape\"").is_err());

        // Invalid: Unterminated string after escape
        assert!(parse_value("\"hello \\").is_err());
    }
}
