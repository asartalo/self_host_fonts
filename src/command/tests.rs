use url::Url;

use crate::command::url_data::{Location, UrlByLine, UrlData};
use pretty_assertions::assert_eq;

use crate::http::get_full_url;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn setup(list: &mut Vec<UrlData>) -> UrlByLine {
    list.push(UrlData {
        url: "https://some.url/font.tff".to_string(),
        location: Location {
            line: 4,
            column: 20,
        },
    });
    list.push(UrlData {
        url: "https://some.url/font2.tff".to_string(),
        location: Location {
            line: 8,
            column: 20,
        },
    });
    list.push(UrlData {
        url: "https://some.url/font3.tff".to_string(),
        location: Location {
            line: 8,
            column: 40,
        },
    });
    UrlByLine::new(list)
}

#[test]
fn ubl_at_returns_none_when_line_number_does_not_match() -> TestResult {
    let mut list: Vec<UrlData> = Vec::new();
    let ubl = setup(&mut list);
    assert_eq!(None, ubl.at(0));
    Ok(())
}

#[test]
fn ubl_at_returns_some_with_matching_line_number() -> TestResult {
    let mut list: Vec<UrlData> = Vec::new();
    let ubl = setup(&mut list);
    let data = ubl.at(4).unwrap();
    let datum = &data[0];
    assert_eq!("https://some.url/font.tff".to_string(), datum.url);
    Ok(())
}

#[test]
fn get_full_url_for_absolute_urls() -> TestResult {
    let font_url = "https://some.url/font.ttf";
    let css_url = Url::parse("https://foo.example/fonts.css").unwrap();
    let result = get_full_url(font_url, &css_url)?;
    assert_eq!(result, Url::parse("https://some.url/font.ttf")?);
    Ok(())
}

#[test]
fn get_full_url_for_root_relative_urls() -> TestResult {
    let font_url = "/assets/font.ttf";
    let css_url = Url::parse("https://foo.example/fonts.css").unwrap();
    let result = get_full_url(font_url, &css_url)?;
    assert_eq!(result, Url::parse("https://foo.example/assets/font.ttf")?);
    Ok(())
}

#[test]
fn get_full_url_for_basic_relative_urls() -> TestResult {
    let font_url = "fonts/font.ttf";
    let css_url = Url::parse("https://foo.example/assets/fonts.css").unwrap();
    let result = get_full_url(font_url, &css_url)?;
    assert_eq!(
        result,
        Url::parse("https://foo.example/assets/fonts/font.ttf")?
    );
    Ok(())
}

#[test]
fn get_full_url_for_basic_relative_urls_with_dots() -> TestResult {
    let font_url = "../fonts/font.ttf";
    let css_url = Url::parse("https://foo.example/assets/fonts.css").unwrap();
    let result = get_full_url(font_url, &css_url)?;
    assert_eq!(result, Url::parse("https://foo.example/fonts/font.ttf")?);
    Ok(())
}

#[test]
fn get_full_url_for_basic_relative_urls_with_two_dots() -> TestResult {
    let font_url = "../../font.ttf";
    let css_url = Url::parse("https://foo.example/assets/css/fonts.css").unwrap();
    let result = get_full_url(font_url, &css_url)?;
    assert_eq!(result, Url::parse("https://foo.example/font.ttf")?);
    Ok(())
}
