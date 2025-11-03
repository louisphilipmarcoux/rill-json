#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
}

fn parse_null(input: &str) -> Result<(JsonValue, &str), &'static str> {
    if input.starts_with("null") {
        Ok((JsonValue::Null, &input[4..]))
    } else {
        Err("Expected 'null'")
    }
}

fn main() {
    // --- Test 1: Valid 'null' --- 
    let input = "null";
    println!("Parsing: '{}'", input);
    match parse_null(input) {
        Ok((value, rest)) => {
            println!("   -> Parsed: {:?}", value);
            println!("   -> Remaining: '{}'", rest);
        }
        Err(e) => println!("   -> Error: {}", e),
    }

    // --- Test 2: Invalid 'nul' --- 
    let invalid_input = "nul";
    println!("\nParsing: '{}'", invalid_input);
    match parse_null(invalid_input) {
        Ok((value, _)) => println!("   -> Error: Incorrectly parsed {:?}", value),
        Err(e) => println!("   -> Correctly failed with: {}", e),
    }

    // --- Test 3: Invalid 'NULL' --- 
    let invalid_input_case = "NULL";
    println!("\nParsing: '{}'", invalid_input_case);
    match parse_null(invalid_input_case) {
        Ok((value, _)) => println!("   -> Error: Incorrectly parsed {:?}", value),
        Err(e) => println!("   -> Correctly failed with: {}", e),
    }

    // --- Test 4: Valid 'null' with extra data (for later stages) ---
    let input_with_extra = "null, \"more stuff\"";
    println!("\nParsing: '{}'", input_with_extra);
    match parse_null(input_with_extra) {
        Ok((value, rest)) => {
            println!("   -> Parsed: {:?}", value);
            println!("   -> Remaining: '{}'", rest);
        }
        Err(e) => println!("   -> Error: {}", e),
    }
}
