//! Contains the `JsonValue` enum, a native Rust representation of any
//! valid JSON value.
//!
//! This module also includes the "stringify" (serialization) logic
//! for converting a `JsonValue` back into a JSON string.
use std::collections::HashMap;
use std::fmt;

// --- 5. JSON Value Enum (for Stage 16) ---

/// A native Rust representation of any valid JSON value.
///
/// This enum is used by the `stringify` functions to serialize
/// Rust data *into* a JSON string.
#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    /// Represents a JSON `null`.
    Null,
    /// Represents a JSON `true` or `false`.
    Boolean(bool),
    /// Represents a JSON number (stored as `f64`).
    Number(f64),
    /// Represents a JSON string.
    String(String),
    /// Represents a JSON array (list).
    Array(Vec<JsonValue>),
    /// Represents a JSON object (map).
    Object(HashMap<String, JsonValue>),
}

// --- 7. Stringify (Serialization - Stage 16) ---
impl JsonValue {
    /// Serializes the `JsonValue` into a compact, minified JSON string.
    ///
    /// # Examples
    /// ```
    /// use rill_json::JsonValue;
    /// let val = JsonValue::Number(123.0);
    /// assert_eq!(val.stringify(), "123");
    /// ```
    pub fn stringify(&self) -> String {
        let mut output = String::new();
        // This unwrap is safe because writing to a String never fails.
        Self::write_value(self, &mut output).unwrap();
        output
    }

    /// Recursive helper function to write any `JsonValue` to a string buffer.
    fn write_value<W: fmt::Write>(value: &JsonValue, w: &mut W) -> fmt::Result {
        match value {
            JsonValue::Null => w.write_str("null"),
            JsonValue::Boolean(b) => w.write_str(if *b { "true" } else { "false" }),
            JsonValue::Number(n) => write!(w, "{}", n),
            JsonValue::String(s) => Self::write_string(s, w),
            JsonValue::Array(a) => Self::write_array(a, w),
            JsonValue::Object(o) => Self::write_object(o, w),
        }
    }

    /// Helper to write a JSON array (compact).
    fn write_array<W: fmt::Write>(arr: &Vec<JsonValue>, w: &mut W) -> fmt::Result {
        w.write_char('[')?;
        let mut first = true;
        for val in arr {
            if !first {
                w.write_char(',')?;
            }
            Self::write_value(val, w)?;
            first = false;
        }
        w.write_char(']')
    }

    /// Helper to write a JSON object (compact).
    fn write_object<W: fmt::Write>(obj: &HashMap<String, JsonValue>, w: &mut W) -> fmt::Result {
        w.write_char('{')?;
        let mut first = true;
        // Note: HashMap iteration order is not guaranteed,
        // but this is fine according to the JSON specification.
        for (key, val) in obj {
            if !first {
                w.write_char(',')?;
            }
            Self::write_string(key, w)?; // Write the key (which must be a string)
            w.write_char(':')?;
            Self::write_value(val, w)?; // Write the value
            first = false;
        }
        w.write_char('}')
    }

    /// Helper to write an escaped JSON string.
    /// This handles all required JSON escape sequences (e.g., `\"`, `\\`, `\n`).
    fn write_string<W: fmt::Write>(s: &str, w: &mut W) -> fmt::Result {
        w.write_char('"')?;
        for c in s.chars() {
            match c {
                // Standard escapes
                '"' => w.write_str("\\\""),
                '\\' => w.write_str("\\\\"),
                '/' => w.write_str("\\/"), // Optional, but good practice
                '\u{0008}' => w.write_str("\\b"), // Backspace
                '\u{000C}' => w.write_str("\\f"), // Form feed
                '\n' => w.write_str("\\n"), // Newline
                '\r' => w.write_str("\\r"), // Carriage return
                '\t' => w.write_str("\\t"), // Tab
                // Control characters must be escaped as \uXXXX
                '\u{0000}'..='\u{001F}' => {
                    write!(w, "\\u{:04x}", c as u32)
                }
                _ => w.write_char(c),
            }?;
        }
        w.write_char('"')
    }

    // --- Pretty Print Bonus ---

    /// The indentation string to use for pretty-printing (two spaces).
    const INDENT: &'static str = "  ";

    /// Serializes the `JsonValue` into a human-readable,
    /// indented JSON string ("pretty-print").
    ///
    /// # Examples
    /// ```
    /// use rill_json::JsonValue;
    /// use std::collections::HashMap;
    ///
    /// let mut obj = HashMap::new();
    /// obj.insert("key".to_string(), JsonValue::String("value".to_string()));
    /// let val = JsonValue::Object(obj);
    ///
    /// let pretty = val.stringify_pretty();
    /// assert!(pretty.starts_with("{\n"));
    /// assert!(pretty.contains("\n  \"key\": \"value\"\n"));
    /// assert!(pretty.ends_with("\n}"));
    /// ```
    pub fn stringify_pretty(&self) -> String {
        let mut output = String::new();
        // This unwrap is safe because writing to a String never fails.
        Self::write_value_pretty(self, &mut output, 0).unwrap();
        output
    }

    /// Recursive helper for pretty-printing a value.
    fn write_value_pretty<W: fmt::Write>(
        value: &JsonValue,
        w: &mut W,
        depth: usize,
    ) -> fmt::Result {
        match value {
            // Primitives are written the same as compact
            JsonValue::Null => w.write_str("null"),
            JsonValue::Boolean(b) => w.write_str(if *b { "true" } else { "false" }),
            JsonValue::Number(n) => write!(w, "{}", n),
            JsonValue::String(s) => Self::write_string(s, w),
            // Composites (Array/Object) get new logic
            JsonValue::Array(a) => Self::write_array_pretty(a, w, depth),
            JsonValue::Object(o) => Self::write_object_pretty(o, w, depth),
        }
    }

    /// Helper to pretty-print a JSON array.
    fn write_array_pretty<W: fmt::Write>(
        arr: &Vec<JsonValue>,
        w: &mut W,
        depth: usize,
    ) -> fmt::Result {
        // Empty array is just "[]"
        if arr.is_empty() {
            return w.write_str("[]");
        }

        let new_depth = depth + 1;
        let indent = Self::INDENT.repeat(new_depth);
        let closing_indent = Self::INDENT.repeat(depth);

        w.write_str("[\n")?; // Opening bracket and newline

        let mut first = true;
        for val in arr {
            if !first {
                w.write_str(",\n")?; // Comma and newline before next item
            }
            w.write_str(&indent)?; // Indent
            Self::write_value_pretty(val, w, new_depth)?; // Write the value
            first = false;
        }

        write!(w, "\n{}", closing_indent)?; // Newline and closing indent
        w.write_char(']') // Closing bracket
    }

    /// Helper to pretty-print a JSON object.
    fn write_object_pretty<W: fmt::Write>(
        obj: &HashMap<String, JsonValue>,
        w: &mut W,
        depth: usize,
    ) -> fmt::Result {
        // Empty object is just "{}"
        if obj.is_empty() {
            return w.write_str("{}");
        }

        let new_depth = depth + 1;
        let indent = Self::INDENT.repeat(new_depth);
        let closing_indent = Self::INDENT.repeat(depth);

        w.write_str("{\n")?; // Opening brace and newline

        let mut first = true;
        for (key, val) in obj {
            if !first {
                w.write_str(",\n")?; // Comma and newline before next item
            }
            w.write_str(&indent)?; // Indent
            Self::write_string(key, w)?; // Write the key
            w.write_str(": ")?; // Colon and space
            Self::write_value_pretty(val, w, new_depth)?; // Write the value
            first = false;
        }

        write!(w, "\n{}", closing_indent)?; // Newline and closing indent
        w.write_char('}') // Closing brace
    }
}
