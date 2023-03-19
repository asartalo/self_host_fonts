use crate::result::CommonResult;
use itertools::Itertools;
use minreq::Response;
use rand::distributions::{Alphanumeric, DistString};
use std::fmt;
use std::fs;
use std::io::Write;
use std::iter::DoubleEndedIterator;
use std::path::Path;
use url::Url;

pub(crate) fn base_url(full_url: &Url) -> CommonResult<Url> {
    let mut url = full_url.clone();
    if let Ok(mut path) = url.path_segments_mut() {
        path.clear();
    }

    url.set_fragment(None);
    url.set_query(None);

    Ok(url)
}

fn url_file_name(url: &Url) -> String {
    if let Some(mut segments) = url.path_segments() {
        if let Some(last) = segments.next_back() {
            if !last.is_empty() {
                return last.to_string();
            }
        }
    }
    Alphanumeric.sample_string(&mut rand::thread_rng(), 10)
}

#[derive(Debug)]
enum CssResponseError {
    NotFoundError { url: String },
    ServerError { url: String },
}

impl std::error::Error for CssResponseError {}

impl fmt::Display for CssResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFoundError { url } => write!(f, "CSS File Not Found: {}", url),
            Self::ServerError { url } => write!(f, "Error retrieving CSS file: {}", url),
        }
    }
}

fn check_response(response: &Response, path: &str) -> Result<(), CssResponseError> {
    let status = response.status_code;
    let url = path.to_string();
    if status == 404 {
        return Err(CssResponseError::NotFoundError { url });
    } else if status >= 400 {
        return Err(CssResponseError::ServerError { url });
    }
    Ok(())
}

fn concat_paths<'a>(from: &'a str, relative_path: &'a str) -> String {
    let mut base_path_parts = from.split('/');

    base_path_parts.next_back();

    format!("{}{}{}", base_path_parts.join("/"), "/", relative_path)
}

pub(crate) fn get_full_url(font_url: &str, css_url: &Url) -> CommonResult<Url> {
    let full_url = if font_url.starts_with("http://") || font_url.starts_with("https://") {
        Url::parse(font_url)?
    } else if font_url.starts_with('/') && !font_url.starts_with("//") {
        let base = base_url(css_url)?;
        let stripped = match font_url.strip_prefix('/') {
            Some(str) => str,
            None => font_url,
        };
        Url::parse(&(base.as_str().to_owned() + stripped))?
    } else {
        let mut base_url = css_url.clone();
        base_url.set_path(&concat_paths(base_url.path(), font_url));
        base_url.join(font_url)?;
        base_url
    };

    Ok(full_url)
}

pub(crate) fn get_css_file(path: &str) -> CommonResult<Response> {
    let request = minreq::get(path)
        .with_header("Accept", "text/css,*/*;q=0.1")
        .with_header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/111.0",
        );

    let css_response = request.send()?;
    check_response(&css_response, path)?;
    Ok(css_response)
}

pub(crate) fn download_file(full_url: Url, output_dir: &Path) -> CommonResult<String> {
    let response = minreq::get(full_url.to_string()).send()?;
    let file_name = url_file_name(&full_url);

    let file_path = output_dir.join(&file_name);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)?;
    file.write_all(response.as_bytes())?;
    Ok(file_name)
}
