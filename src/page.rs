//! Typed, validated page metadata parsed from TOML frontmatter.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use time::{Date, Month, macros::format_description};

use crate::error::{AppError, AppResult};

const DATE_FORMAT: &[time::format_description::BorrowedFormatItem<'static>] =
    format_description!("[year]-[month]-[day]");
const RESERVED_FIELDS: &[&str] = &[
    "title",
    "description",
    "date",
    "updated",
    "draft",
    "layout",
    "slug",
    "tags",
    "projects",
    "canonical_url",
    "social_image",
    "extra",
];

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub source: PathBuf,
    pub metadata: PageMetadata,
    pub body: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<Date>,
    pub updated: Option<Date>,
    pub draft: bool,
    pub layout: Option<String>,
    pub slug: Option<String>,
    pub tags: Vec<String>,
    pub projects: Vec<String>,
    pub canonical_url: Option<String>,
    pub social_image: Option<String>,
    pub extra: BTreeMap<String, toml::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildProfile {
    Production { as_of: Date },
    Development { as_of: Date },
}

impl BuildProfile {
    pub fn production(as_of: Date) -> Self {
        Self::Production { as_of }
    }

    pub fn development(as_of: Date) -> Self {
        Self::Development { as_of }
    }

    pub fn includes(self, page: &Page) -> bool {
        match self {
            Self::Production { as_of } => {
                !page.metadata.draft && page.metadata.date.is_none_or(|date| date <= as_of)
            }
            Self::Development { .. } => true,
        }
    }

    pub fn shows_draft_marker(self, page: &Page) -> bool {
        matches!(self, Self::Development { .. }) && page.metadata.draft
    }
}

pub fn parse(source: &Path, bytes: &[u8]) -> AppResult<Page> {
    let text = std::str::from_utf8(bytes).map_err(|error| {
        diagnostic_at(
            source,
            bytes,
            error.valid_up_to(),
            "",
            "valid UTF-8",
            "content is not valid Unicode".into(),
        )
    })?;
    let (metadata, body) = split_frontmatter(source, text)?;
    Ok(Page {
        source: source.to_path_buf(),
        metadata,
        body: body.to_owned(),
    })
}

fn split_frontmatter<'a>(source: &Path, text: &'a str) -> AppResult<(PageMetadata, &'a str)> {
    if !text.starts_with("+++") {
        return Ok((PageMetadata::default(), text));
    }
    let Some(rest) = text.strip_prefix("+++\n") else {
        return Err(diagnostic_at(
            source,
            text.as_bytes(),
            0,
            "",
            "an opening delimiter exactly equal to +++ followed by a newline",
            "malformed frontmatter delimiter".into(),
        ));
    };
    let Some(end) = rest.find("\n+++\n") else {
        return Err(diagnostic_at(
            source,
            text.as_bytes(),
            0,
            "",
            "a closing delimiter exactly equal to +++",
            "frontmatter is not closed".into(),
        ));
    };
    let header = &rest[..end];
    let body = &rest[end + "\n+++\n".len()..];
    Ok((parse_metadata(source, header)?, body))
}

fn parse_metadata(source: &Path, header: &str) -> AppResult<PageMetadata> {
    let table: toml::Table = toml::from_str(header).map_err(|error| {
        diagnostic_at(
            source,
            header.as_bytes(),
            error.span().map_or(0, |span| span.start),
            "",
            "valid TOML frontmatter",
            error.to_string(),
        )
    })?;
    for field in table.keys() {
        if !RESERVED_FIELDS.contains(&field.as_str()) {
            return Err(diagnostic_at(
                source,
                header.as_bytes(),
                find_field_offset(header, field),
                field,
                "a documented reserved field or a value below extra",
                unknown_field_message(field),
            ));
        }
    }
    Ok(PageMetadata {
        title: optional_string(source, header, &table, "title")?,
        description: optional_string(source, header, &table, "description")?,
        date: optional_date(source, header, &table, "date")?,
        updated: optional_date(source, header, &table, "updated")?,
        draft: optional_bool(source, header, &table, "draft")?.unwrap_or(false),
        layout: optional_string(source, header, &table, "layout")?,
        slug: optional_string(source, header, &table, "slug")?,
        tags: optional_string_array(source, header, &table, "tags")?.unwrap_or_default(),
        projects: optional_string_array(source, header, &table, "projects")?.unwrap_or_default(),
        canonical_url: optional_string(source, header, &table, "canonical_url")?,
        social_image: optional_string(source, header, &table, "social_image")?,
        extra: optional_table(source, header, &table, "extra")?.unwrap_or_default(),
    })
}

fn optional_string(
    source: &Path,
    header: &str,
    table: &toml::Table,
    field: &str,
) -> AppResult<Option<String>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };
    value
        .as_str()
        .map(ToOwned::to_owned)
        .map(Some)
        .ok_or_else(|| wrong_type(source, header, field, "a string"))
}
fn optional_bool(
    source: &Path,
    header: &str,
    table: &toml::Table,
    field: &str,
) -> AppResult<Option<bool>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };
    value
        .as_bool()
        .map(Some)
        .ok_or_else(|| wrong_type(source, header, field, "a boolean"))
}
fn optional_date(
    source: &Path,
    header: &str,
    table: &toml::Table,
    field: &str,
) -> AppResult<Option<Date>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };
    let Some(value) = value.as_str() else {
        return Err(wrong_type(source, header, field, "a YYYY-MM-DD string"));
    };
    Date::parse(value, DATE_FORMAT).map(Some).map_err(|error| {
        diagnostic_at(
            source,
            header.as_bytes(),
            find_field_offset(header, field),
            field,
            "a YYYY-MM-DD calendar date",
            error.to_string(),
        )
    })
}
fn optional_string_array(
    source: &Path,
    header: &str,
    table: &toml::Table,
    field: &str,
) -> AppResult<Option<Vec<String>>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };
    let Some(values) = value.as_array() else {
        return Err(wrong_type(source, header, field, "an array of strings"));
    };
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| wrong_type(source, header, field, "an array of strings"))
        })
        .collect::<AppResult<Vec<_>>>()
        .map(Some)
}
fn optional_table(
    source: &Path,
    header: &str,
    table: &toml::Table,
    field: &str,
) -> AppResult<Option<BTreeMap<String, toml::Value>>> {
    let Some(value) = table.get(field) else {
        return Ok(None);
    };
    value
        .as_table()
        .map(|values| values.clone().into_iter().collect())
        .map(Some)
        .ok_or_else(|| wrong_type(source, header, field, "a TOML table"))
}
fn wrong_type(source: &Path, header: &str, field: &str, expected: &'static str) -> AppError {
    diagnostic_at(
        source,
        header.as_bytes(),
        find_field_offset(header, field),
        field,
        expected,
        "field has an incompatible value".into(),
    )
}
fn unknown_field_message(field: &str) -> String {
    let suggestion = RESERVED_FIELDS
        .iter()
        .min_by_key(|candidate| edit_distance(field, candidate));
    match suggestion.filter(|candidate| edit_distance(field, candidate) <= 3) {
        Some(suggestion) => format!("unknown reserved-looking field; did you mean `{suggestion}`?"),
        None => "unknown field; move user-defined metadata below `extra`".into(),
    }
}
fn find_field_offset(header: &str, field: &str) -> usize {
    header.find(field).unwrap_or(0)
}
fn diagnostic_at(
    source: &Path,
    text: &[u8],
    offset: usize,
    field: &str,
    expected: &'static str,
    message: String,
) -> AppError {
    let safe_offset = offset.min(text.len());
    let line = text[..safe_offset]
        .iter()
        .filter(|byte| **byte == b'\n')
        .count()
        + 1;
    let column = safe_offset
        - text[..safe_offset]
            .iter()
            .rposition(|byte| *byte == b'\n')
            .map_or(0, |index| index + 1)
        + 1;
    AppError::Frontmatter {
        input: source.to_path_buf(),
        line,
        column,
        field: if field.is_empty() {
            String::new()
        } else {
            format!(" for field `{field}`")
        },
        expected,
        message,
    }
}
fn edit_distance(left: &str, right: &str) -> usize {
    let mut costs: Vec<usize> = (0..=right.len()).collect();
    for (i, left) in left.bytes().enumerate() {
        let mut previous = costs[0];
        costs[0] = i + 1;
        for (j, right) in right.bytes().enumerate() {
            let current = costs[j + 1];
            costs[j + 1] = (costs[j + 1] + 1)
                .min(costs[j] + 1)
                .min(previous + usize::from(left != right));
            previous = current;
        }
    }
    costs[right.len()]
}

pub fn calendar_date(year: i32, month: u8, day: u8) -> Date {
    Date::from_calendar_date(year, Month::try_from(month).expect("valid month"), day)
        .expect("valid date")
}
