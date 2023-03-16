mod cli;
#[cfg(test)]
mod tests;
mod url_data;

use crate::{
    command::{
        cli::{Cli, Parser},
        url_data::{get_url_data, UrlByLine},
    },
    http,
    read::LinesWithEndings,
    result::CommonResult,
};

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

fn get_output_dir(cli: &Cli) -> CommonResult<PathBuf> {
    match &cli.dir {
        Some(dir) => Ok(dir.clone()),
        None => Ok(env::current_dir()?),
    }
}

pub(crate) fn run() -> CommonResult<()> {
    let cli = Cli::parse();
    let path = &cli.css_path;
    let font_url_prefix = &cli.font_url_prefix;
    let output_dir = get_output_dir(&cli)?;

    println!("Loading {}", path);
    let css_response = http::get_css_file(path)?;
    let css_str = css_response.as_str()?;

    let font_urls = get_url_data(css_str.to_owned())?;

    println!("Found {} font declarations", font_urls.len());
    let css_url = Url::parse(path)?;
    let base = http::base_url(&css_url)?;
    let mut replacements: HashMap<&String, String> = HashMap::new();

    for font_url_data in &font_urls {
        let font_url = &font_url_data.url;
        let full_url = if font_url.starts_with("http://") || font_url.starts_with("https://") {
            Url::parse(font_url)?
        } else if font_url.starts_with('/') && !font_url.starts_with("//") {
            let stripped = match font_url.strip_prefix('/') {
                Some(str) => str,
                None => font_url,
            };
            Url::parse(&(base.as_str().to_owned() + stripped))?
        } else {
            let base_url = Url::parse(path)?;
            base_url.join(font_url)?;
            base_url
        };

        println!("Downloading {}", full_url);
        let file_name = http::download_file(full_url, &output_dir)?;
        replacements.insert(font_url, format!("{}{}", font_url_prefix, file_name));
    }

    let css_file_path = output_dir.join("fonts.css");
    println!("Writing updated css file: {}", css_file_path.display());
    let mut css_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(css_file_path)?;

    let urls_by_line = UrlByLine::new(&font_urls);

    let mut line_number: u32 = 0;
    let lines = LinesWithEndings::from(css_response.as_str()?);
    for line_result in lines {
        line_number += 1;
        let line = line_result;

        if let Some(items) = urls_by_line.at(line_number) {
            for item in items.iter().rev() {
                let mut line_clone = line.to_owned();
                let start = usize::try_from(item.location.column)? + 3;
                let end = start + item.url.len();
                line_clone.replace_range(start..end, replacements.get(&item.url).unwrap());

                css_file.write_all(line_clone.as_bytes())?;
            }
        } else {
            css_file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}
