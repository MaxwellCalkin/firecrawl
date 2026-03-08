//! Helpers for deserializing metadata fields that may arrive as either
//! a single JSON string **or** an array of strings from the Firecrawl API.
//!
//! Some HTML pages contain duplicate meta tags (e.g. multiple `viewport` or
//! `twitter:image` tags). The scraper collects them into a JSON array, but
//! the Rust SDK previously expected every metadata value to be a plain
//! string, causing `serde_json` to fail with:
//!
//! > invalid type: sequence, expected a string
//!
//! The [`deserialize_flexible_string`] function accepts both shapes and
//! joins arrays with `", "` so existing call-sites that expect
//! `Option<String>` keep working.

use serde::de;
use serde_json::Value;

/// Deserialise an `Option<String>` from a JSON value that may be a string,
/// an array of strings, or `null`.
///
/// *  `"hello"`           -> `Some("hello")`
/// *  `["a", "b"]`        -> `Some("a, b")`
/// *  `null` / absent key -> `None`
pub fn deserialize_flexible_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value: Option<Value> = de::Deserialize::deserialize(deserializer)?;
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s)),
        Some(Value::Array(arr)) => {
            let strings: Vec<String> = arr
                .into_iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s),
                    other => Ok(other.to_string()),
                })
                .collect::<Result<Vec<_>, D::Error>>()?;
            if strings.is_empty() {
                Ok(None)
            } else {
                Ok(Some(strings.join(", ")))
            }
        }
        Some(other) => Ok(Some(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct TestStruct {
        #[serde(default, deserialize_with = "deserialize_flexible_string")]
        field: Option<String>,
    }

    #[test]
    fn test_string_value() {
        let json = r#"{"field": "hello"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.field, Some("hello".to_string()));
    }

    #[test]
    fn test_array_value() {
        let json = r#"{"field": ["a", "b", "c"]}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.field, Some("a, b, c".to_string()));
    }

    #[test]
    fn test_null_value() {
        let json = r#"{"field": null}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.field, None);
    }

    #[test]
    fn test_missing_value() {
        let json = r#"{}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.field, None);
    }

    #[test]
    fn test_single_element_array() {
        let json = r#"{"field": ["only"]}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.field, Some("only".to_string()));
    }

    #[test]
    fn test_empty_array() {
        let json = r#"{"field": []}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.field, None);
    }

    #[test]
    fn test_numeric_value() {
        let json = r#"{"field": 42}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.field, Some("42".to_string()));
    }
}
