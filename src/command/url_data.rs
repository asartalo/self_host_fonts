use crate::result::CommonResult;
use lightningcss::{
    rules::{
        font_face::{FontFaceProperty::Source, FontFaceRule, Source as SourceEnum},
        CssRule::FontFace,
    },
    stylesheet::{ParserOptions, StyleSheet},
};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Location {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct UrlData {
    pub url: String,
    pub location: Location,
}

pub(crate) struct UrlByLine {
    url_data: HashMap<u32, Vec<UrlData>>,
}

impl UrlByLine {
    pub fn new(data: &Vec<UrlData>) -> UrlByLine {
        let mut map: HashMap<u32, Vec<UrlData>> = HashMap::new();
        for item in data {
            map.insert(item.location.line, Vec::new());
        }
        for item in data {
            if let Some(items) = map.get_mut(&item.location.line) {
                items.push(item.clone());
            }
        }

        UrlByLine { url_data: map }
    }

    pub fn at(&self, line: u32) -> Option<&Vec<UrlData>> {
        self.url_data.get(&line)
    }
}

#[must_use]
pub(crate) fn get_font_url(rule: &FontFaceRule) -> Option<UrlData> {
    for property in &rule.properties {
        if let Source(sources) = property {
            for source in sources {
                if let SourceEnum::Url(url_src) = source {
                    let loc = url_src.url.loc;
                    return Some(UrlData {
                        url: url_src.url.url.to_string(),
                        location: Location {
                            line: loc.line,
                            column: loc.column,
                        },
                    });
                }
            }
        }
    }
    None
}

#[derive(Debug)]
pub(crate) struct GetUrlDataError {
    v: String,
}

impl std::fmt::Display for GetUrlDataError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.v, f)
    }
}

impl Error for GetUrlDataError {}

pub(crate) fn get_url_data(css_str: String) -> CommonResult<Vec<UrlData>> {
    let mut font_urls: Vec<UrlData> = Vec::new();

    let stylesheet =
        StyleSheet::parse(&css_str, ParserOptions::default()).map_err(|err| GetUrlDataError {
            v: format!("CSS Parsing Error: {}", err),
        })?;

    for rule in &stylesheet.rules.0 {
        if let FontFace(ff_rule) = rule {
            if let Some(url) = get_font_url(ff_rule) {
                font_urls.push(url.clone());
            }
        }
    }

    Ok(font_urls)
}
