use std::collections::HashMap;
use std::fmt;

use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// A metadata value that can be either a single string or a list of strings.
///
/// The Firecrawl API returns metadata values as strings for unique meta tags,
/// but as arrays when a page has duplicate meta tags with the same name (e.g.,
/// multiple `viewport` or `twitter:image` tags). This type handles both cases
/// transparently.
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MetadataValue {
    /// A single string value.
    Single(String),
    /// Multiple values from duplicate meta tags.
    Many(Vec<String>),
}

impl MetadataValue {
    /// Returns the value as a single string.
    ///
    /// For `Single`, returns the contained string.
    /// For `Many`, joins the values with `", "`.
    pub fn as_str_lossy(&self) -> String {
        match self {
            MetadataValue::Single(s) => s.clone(),
            MetadataValue::Many(v) => v.join(", "),
        }
    }

    /// Returns the first value, regardless of variant.
    pub fn first(&self) -> &str {
        match self {
            MetadataValue::Single(s) => s,
            MetadataValue::Many(v) => v.first().map(|s| s.as_str()).unwrap_or(""),
        }
    }

    /// Returns all values as a `Vec<String>`.
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            MetadataValue::Single(s) => vec![s.clone()],
            MetadataValue::Many(v) => v.clone(),
        }
    }
}

impl Default for MetadataValue {
    fn default() -> Self {
        MetadataValue::Single(String::new())
    }
}

impl fmt::Display for MetadataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str_lossy())
    }
}

impl From<String> for MetadataValue {
    fn from(s: String) -> Self {
        MetadataValue::Single(s)
    }
}

impl From<Vec<String>> for MetadataValue {
    fn from(v: Vec<String>) -> Self {
        MetadataValue::Many(v)
    }
}

impl<'de> Deserialize<'de> for MetadataValue {
    fn deserialize<D>(deserializer: D) -> Result<MetadataValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetadataValueVisitor;

        impl<'de> Visitor<'de> for MetadataValueVisitor {
            type Value = MetadataValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an array of strings")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<MetadataValue, E> {
                Ok(MetadataValue::Single(value.to_owned()))
            }

            fn visit_string<E: de::Error>(self, value: String) -> Result<MetadataValue, E> {
                Ok(MetadataValue::Single(value))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<MetadataValue, A::Error> {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element::<String>()? {
                    values.push(value);
                }
                Ok(MetadataValue::Many(values))
            }
        }

        deserializer.deserialize_any(MetadataValueVisitor)
    }
}

/// Deserializes an `Option<MetadataValue>` that accepts a string, array of strings, or null.
pub mod option_metadata_value {
    use super::MetadataValue;
    use serde::de::{self, SeqAccess, Visitor};
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<MetadataValue>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OptionalMetadataValueVisitor;

        impl<'de> Visitor<'de> for OptionalMetadataValueVisitor {
            type Value = Option<MetadataValue>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("null, a string, or an array of strings")
            }

            fn visit_none<E: de::Error>(self) -> Result<Option<MetadataValue>, E> {
                Ok(None)
            }

            fn visit_unit<E: de::Error>(self) -> Result<Option<MetadataValue>, E> {
                Ok(None)
            }

            fn visit_some<D2: Deserializer<'de>>(
                self,
                deserializer: D2,
            ) -> Result<Option<MetadataValue>, D2::Error> {
                MetadataValue::deserialize(deserializer).map(Some)
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Option<MetadataValue>, E> {
                Ok(Some(MetadataValue::Single(value.to_owned())))
            }

            fn visit_string<E: de::Error>(
                self,
                value: String,
            ) -> Result<Option<MetadataValue>, E> {
                Ok(Some(MetadataValue::Single(value)))
            }

            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Option<MetadataValue>, A::Error> {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element::<String>()? {
                    values.push(value);
                }
                Ok(Some(MetadataValue::Many(values)))
            }
        }

        deserializer.deserialize_any(OptionalMetadataValueVisitor)
    }

    pub fn serialize<S>(value: &Option<MetadataValue>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serde::Serialize::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMetadata {
    // firecrawl specific
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    pub status_code: u16,
    pub error: Option<String>,

    // basic meta tags
    #[serde(default, with = "option_metadata_value")]
    pub title: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub description: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub language: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub keywords: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub robots: Option<MetadataValue>,

    // og: namespace
    #[serde(default, with = "option_metadata_value")]
    pub og_title: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub og_description: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub og_url: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub og_image: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub og_audio: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub og_determiner: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub og_locale: Option<MetadataValue>,
    pub og_locale_alternate: Option<Vec<String>>,
    #[serde(default, with = "option_metadata_value")]
    pub og_site_name: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub og_video: Option<MetadataValue>,

    // article: namespace
    #[serde(default, with = "option_metadata_value")]
    pub article_section: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub article_tag: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub published_time: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub modified_time: Option<MetadataValue>,

    // dc./dcterms. namespace
    #[serde(default, with = "option_metadata_value")]
    pub dcterms_keywords: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dc_description: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dc_subject: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dcterms_subject: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dcterms_audience: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dc_type: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dcterms_type: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dc_date: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dc_date_created: Option<MetadataValue>,
    #[serde(default, with = "option_metadata_value")]
    pub dcterms_created: Option<MetadataValue>,

    /// Additional metadata fields not covered by the named fields above.
    ///
    /// HTML pages can have arbitrary meta tags (e.g., `viewport`, `twitter:image`,
    /// `theme-color`). These are captured here as raw JSON values, which may be
    /// strings or arrays of strings.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// A list of the links on the page, present if `ScrapeFormats::Markdown` is present in `ScrapeOptions.formats`. (default)
    pub markdown: Option<String>,

    /// The HTML of the page, present if `ScrapeFormats::HTML` is present in `ScrapeOptions.formats`.
    ///
    /// This contains HTML that has non-content tags removed. If you need the original HTML, use `ScrapeFormats::RawHTML`.
    pub html: Option<String>,

    /// The raw HTML of the page, present if `ScrapeFormats::RawHTML` is present in `ScrapeOptions.formats`.
    ///
    /// This contains the original, untouched HTML on the page. If you only need human-readable content, use `ScrapeFormats::HTML`.
    pub raw_html: Option<String>,

    /// The URL to the screenshot of the page, present if `ScrapeFormats::Screenshot` or `ScrapeFormats::ScreenshotFullPage` is present in `ScrapeOptions.formats`.
    pub screenshot: Option<String>,

    /// A list of the links on the page, present if `ScrapeFormats::Links` is present in `ScrapeOptions.formats`.
    pub links: Option<Vec<String>>,

    /// The extracted data from the page, present if `ScrapeFormats::Extract` is present in `ScrapeOptions.formats`.
    /// If `ScrapeOptions.extract.schema` is `Some`, this `Value` is guaranteed to match the provided schema.
    pub extract: Option<Value>,

    /// The metadata from the page.
    pub metadata: DocumentMetadata,

    /// Can be present if `ScrapeFormats::Extract` is present in `ScrapeOptions.formats`.
    /// The warning message will contain any errors encountered during the extraction.
    pub warning: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_metadata_value_from_string() {
        let v: MetadataValue = serde_json::from_value(json!("hello")).unwrap();
        assert_eq!(v, MetadataValue::Single("hello".to_string()));
        assert_eq!(v.as_str_lossy(), "hello");
        assert_eq!(v.first(), "hello");
        assert_eq!(v.to_vec(), vec!["hello".to_string()]);
    }

    #[test]
    fn test_metadata_value_from_array() {
        let v: MetadataValue =
            serde_json::from_value(json!(["width=device-width", "initial-scale=1"])).unwrap();
        assert_eq!(
            v,
            MetadataValue::Many(vec![
                "width=device-width".to_string(),
                "initial-scale=1".to_string()
            ])
        );
        assert_eq!(v.as_str_lossy(), "width=device-width, initial-scale=1");
        assert_eq!(v.first(), "width=device-width");
        assert_eq!(
            v.to_vec(),
            vec![
                "width=device-width".to_string(),
                "initial-scale=1".to_string()
            ]
        );
    }

    #[test]
    fn test_metadata_value_serialization_roundtrip() {
        let single = MetadataValue::Single("test".to_string());
        let json = serde_json::to_value(&single).unwrap();
        assert_eq!(json, json!("test"));

        let many = MetadataValue::Many(vec!["a".to_string(), "b".to_string()]);
        let json = serde_json::to_value(&many).unwrap();
        assert_eq!(json, json!(["a", "b"]));
    }

    #[test]
    fn test_metadata_deserialize_string_fields() {
        let data = json!({
            "sourceURL": "https://example.com",
            "statusCode": 200,
            "title": "Example Page",
            "description": "A test page"
        });
        let meta: DocumentMetadata = serde_json::from_value(data).unwrap();
        assert_eq!(
            meta.title,
            Some(MetadataValue::Single("Example Page".to_string()))
        );
        assert_eq!(
            meta.description,
            Some(MetadataValue::Single("A test page".to_string()))
        );
    }

    #[test]
    fn test_metadata_deserialize_array_fields() {
        // This is the exact scenario from issue #1304: metadata fields returned as arrays
        let data = json!({
            "sourceURL": "https://www.cnn.com/",
            "statusCode": 200,
            "viewport": [
                "width=device-width, initial-scale=1",
                "width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no,viewport-fit=cover"
            ],
            "twitter:image": [
                "http://localhost:3000/fallback.jpg",
                "https://images.example.com/photo.jpg"
            ],
            "title": "CNN - Breaking News",
            "keywords": ["news", "politics", "world"]
        });
        let meta: DocumentMetadata = serde_json::from_value(data).unwrap();

        // title is still a string
        assert_eq!(
            meta.title,
            Some(MetadataValue::Single("CNN - Breaking News".to_string()))
        );

        // keywords came as an array
        assert_eq!(
            meta.keywords,
            Some(MetadataValue::Many(vec![
                "news".to_string(),
                "politics".to_string(),
                "world".to_string(),
            ]))
        );

        // viewport and twitter:image go into extra since they aren't named fields
        assert!(meta.extra.contains_key("viewport"));
        assert!(meta.extra.contains_key("twitter:image"));
        assert_eq!(
            meta.extra["viewport"],
            json!([
                "width=device-width, initial-scale=1",
                "width=device-width,initial-scale=1.0,maximum-scale=1.0,user-scalable=no,viewport-fit=cover"
            ])
        );
    }

    #[test]
    fn test_metadata_deserialize_null_fields() {
        let data = json!({
            "sourceURL": "https://example.com",
            "statusCode": 200,
            "title": null,
            "description": null
        });
        let meta: DocumentMetadata = serde_json::from_value(data).unwrap();
        assert_eq!(meta.title, None);
        assert_eq!(meta.description, None);
    }

    #[test]
    fn test_metadata_deserialize_missing_fields() {
        let data = json!({
            "sourceURL": "https://example.com",
            "statusCode": 200
        });
        let meta: DocumentMetadata = serde_json::from_value(data).unwrap();
        assert_eq!(meta.title, None);
        assert_eq!(meta.keywords, None);
        assert!(meta.extra.is_empty());
    }

    #[test]
    fn test_metadata_extra_captures_unknown_fields() {
        let data = json!({
            "sourceURL": "https://example.com",
            "statusCode": 200,
            "theme-color": "#ffffff",
            "custom-meta": "some value"
        });
        let meta: DocumentMetadata = serde_json::from_value(data).unwrap();
        assert_eq!(meta.extra["theme-color"], json!("#ffffff"));
        assert_eq!(meta.extra["custom-meta"], json!("some value"));
    }
}
