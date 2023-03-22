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

    println!("\nLoading {}", path);
    let css_response = http::get_css_file(path)?;
    let css_str = css_response.as_str()?;

    let font_urls = get_url_data(css_str.to_owned())?;
    let count = font_urls.len();

    let css_url = Url::parse(path)?;
    let mut replacements: HashMap<&String, String> = HashMap::new();

    let mut current = 0;

    let pluralized = if count > 1 { "file" } else { "files" };
    println!("Downloading {} font {}", count, pluralized);
    for font_url_data in &font_urls {
        current += 1;
        let font_url = &font_url_data.url;
        let full_url = http::get_full_url(font_url, &css_url)?;
        let url_str = full_url.to_string();
        let file_name = http::download_file(full_url, &output_dir)?;

        println!(" {}/{}\t{}", current, count, url_str);
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

    println!("Done.");

    Ok(())
}
