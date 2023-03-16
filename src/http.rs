use crate::result::CommonResult;
use minreq::Response;
use rand::distributions::{Alphanumeric, DistString};
use std::fmt;
use std::fs;
use std::io::Write;
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
struct NotFoundError {
    url: String,
}

impl std::error::Error for NotFoundError {}
impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CSS File Not Found: {}", self.url)
    }
}

fn check_response(response: &Response, path: &str) -> Result<(), NotFoundError> {
    if response.status_code == 404 {
        return Err(NotFoundError {
            url: path.to_string(),
        });
    }
    Ok(())
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
