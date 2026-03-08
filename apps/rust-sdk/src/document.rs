use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::flexible_value::deserialize_flexible_string;

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
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub language: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub keywords: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub robots: Option<String>,

    // og: namespace
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_image: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_audio: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_determiner: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_locale: Option<String>,
    pub og_locale_alternate: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_site_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub og_video: Option<String>,

    // article: namespace
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub article_section: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub article_tag: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub published_time: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub modified_time: Option<String>,

    // dc./dcterms. namespace
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dcterms_keywords: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dc_description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dc_subject: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dcterms_subject: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dcterms_audience: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dc_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dcterms_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dc_date: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dc_date_created: Option<String>,
    #[serde(default, deserialize_with = "deserialize_flexible_string")]
    pub dcterms_created: Option<String>,

    /// Additional metadata fields not covered by the struct fields above.
    /// The API may return arbitrary metadata keys with string, array, or other
    /// JSON values (e.g. `"viewport": ["width=...", "width=..."]`).
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
