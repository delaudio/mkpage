//! Deterministic, safe Markdown rendering with structured metadata.

use std::collections::BTreeSet;

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, html};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub url: String,
    pub title: String,
    pub internal: bool,
    pub outbound: bool,
    pub asset: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedMarkdown {
    pub html: String,
    pub headings: Vec<Heading>,
    pub links: Vec<Link>,
    pub assets: Vec<String>,
    pub summary: String,
}

pub fn render(input: &str) -> RenderedMarkdown {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH;
    let events: Vec<_> = Parser::new_ext(input, options).collect();
    let headings = headings(&events);
    let links = links(&events);
    let assets = links
        .iter()
        .filter(|link| link.asset)
        .map(|link| link.url.clone())
        .collect();
    let summary = plain_text(&events);
    let safe_events = events.into_iter().map(safe_event);
    let mut output = String::new();
    html::push_html(&mut output, safe_events);
    for heading in &headings {
        let needle = format!("<h{}>", heading.level);
        let replacement = format!("<h{} id=\"{}\">", heading.level, heading.id);
        output = output.replacen(&needle, &replacement, 1);
    }
    output = neutralize_unsafe_schemes(output);
    RenderedMarkdown {
        html: output,
        headings,
        links,
        assets,
        summary,
    }
}

fn neutralize_unsafe_schemes(mut html: String) -> String {
    for scheme in ["javascript:", "data:", "vbscript:"] {
        html = html.replace(scheme, "#");
        html = html.replace(&scheme.to_ascii_uppercase(), "#");
    }
    html
}

fn safe_event(event: Event<'_>) -> Event<'_> {
    match event {
        Event::Html(value) | Event::InlineHtml(value) => Event::Text(value),
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) if unsafe_scheme(&dest_url) => Event::Start(Tag::Link {
            link_type,
            dest_url: CowStr::Borrowed("#"),
            title,
            id,
        }),
        event => event,
    }
}

fn headings(events: &[Event<'_>]) -> Vec<Heading> {
    let mut result = Vec::new();
    let mut current: Option<(u8, String)> = None;
    let mut used = BTreeSet::new();
    for event in events {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current = Some((*level as u8, String::new()))
            }
            Event::Text(text) | Event::Code(text) if current.is_some() => {
                current.as_mut().unwrap().1.push_str(text)
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, text)) = current.take() {
                    let base = slugify(&text);
                    let mut id = base.clone();
                    let mut number = 2;
                    while !used.insert(id.clone()) {
                        id = format!("{base}-{number}");
                        number += 1;
                    }
                    result.push(Heading { level, id, text });
                }
            }
            _ => {}
        }
    }
    result
}

fn links(events: &[Event<'_>]) -> Vec<Link> {
    events
        .iter()
        .filter_map(|event| match event {
            Event::Start(Tag::Link {
                dest_url, title, ..
            }) => Some(link(dest_url, title)),
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => Some(Link {
                url: dest_url.to_string(),
                title: title.to_string(),
                internal: !is_external(dest_url),
                outbound: is_external(dest_url),
                asset: true,
            }),
            _ => None,
        })
        .collect()
}
fn link(url: &str, title: &str) -> Link {
    let external = is_external(url);
    Link {
        url: url.to_owned(),
        title: title.to_owned(),
        internal: !external && !unsafe_scheme(url),
        outbound: external,
        asset: false,
    }
}
fn plain_text(events: &[Event<'_>]) -> String {
    events
        .iter()
        .filter_map(|event| match event {
            Event::Text(text) | Event::Code(text) => Some(text.as_ref()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn unsafe_scheme(url: &str) -> bool {
    url.split(':').next().is_some_and(|scheme| {
        matches!(
            scheme.to_ascii_lowercase().as_str(),
            "javascript" | "data" | "vbscript"
        )
    })
}
fn is_external(url: &str) -> bool {
    matches!(url.split(':').next().map(|scheme| scheme.to_ascii_lowercase()), Some(scheme) if scheme == "http" || scheme == "https" || scheme == "mailto")
}
fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut dash = false;
    for character in input.chars() {
        if character.is_alphanumeric() {
            slug.extend(character.to_lowercase());
            dash = false;
        } else if !dash && !slug.is_empty() {
            slug.push('-');
            dash = true;
        }
    }
    slug.trim_matches('-').to_owned().if_empty("section")
}
trait IfEmpty {
    fn if_empty(self, fallback: &str) -> Self;
}
impl IfEmpty for String {
    fn if_empty(self, fallback: &str) -> Self {
        if self.is_empty() {
            fallback.into()
        } else {
            self
        }
    }
}
