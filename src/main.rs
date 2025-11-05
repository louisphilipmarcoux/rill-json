//! A binary executable that demonstrates how to use the `rill-json` library.
//!
//! This is not part of the library itself, but provides a simple
//! example of both parsing and stringifying JSON.
//!
//! You can run this example with: `cargo run`

use rill_json::{parse_streaming, JsonValue};
use std::collections::HashMap;

fn main() {
    let input = "{ \"key\": [1, true, null] }";
    println!("--- Running Streaming Parser ---");
    println!("Parsing: {}", input);

    // Call the library function to get a parser iterator
    match parse_streaming(input) {
        Ok(parser) => {
            // Iterate over all parser events
            for event in parser {
                match event {
                    Ok(event) => println!("Event: {:?}", event),
                    Err(e) => {
                        println!("{}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => println!("{}", e),
    }

    println!("\n--- Running Stringify Demo ---");

    // 1. Build a native Rust data structure
    let mut items = HashMap::new();
    items.insert("key".to_string(), JsonValue::String("value".to_string()));
    items.insert(
        "items".to_string(),
        JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Null]),
    );
    let obj = JsonValue::Object(items);
    println!("Serializing: {:?}", obj);

    // 2. Call the library functions to serialize it
    println!("Compact: {}", obj.stringify());
    println!("Pretty:\n{}", obj.stringify_pretty());
}
