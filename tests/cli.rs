use assert_cmd::Command;
use httpmock::prelude::*;
use pretty_assertions::assert_eq;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

type TestResult = Result<(), Box<dyn std::error::Error>>;
type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn file_contents(file_path: &str) -> MyResult<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
    Ok(())
}

#[test]
fn fails_when_css_does_not_exist() -> TestResult {
    let server = MockServer::start();
    let css_request_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/css")
            .query_param("family", "Roboto:300,300i,400")
            .matches(|req| req.path == "/css");
        then.status(404);
    });

    let css_url = server.url("/css?family=Roboto:300,300i,400");

    let dir = Path::new("./tests/tmp");
    let working_dir = get_current_working_dir()?;

    // Remove the output directory if it already exists
    if dir.exists() && dir.starts_with(working_dir) {
        fs::remove_dir_all(dir)?;
    }

    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.arg(css_url)
        .arg(format!("--dir={}", dir.to_str().unwrap()))
        .assert()
        .failure()
        .stderr(predicates::str::contains("CSS File Not Found"));

    css_request_mock.assert();
    Ok(())
}

#[test]
fn fails_when_css_request_returns_not_ok() -> TestResult {
    let server = MockServer::start();
    let css_request_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/css")
            .query_param("family", "Roboto:300,300i,400")
            .matches(|req| req.path == "/css");
        then.status(500);
    });

    let css_url = server.url("/css?family=Roboto:300,300i,400");

    let dir = Path::new("./tests/tmp");
    let working_dir = get_current_working_dir()?;

    // Remove the output directory if it already exists
    if dir.exists() && dir.starts_with(working_dir) {
        fs::remove_dir_all(dir)?;
    }

    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.arg(css_url)
        .arg(format!("--dir={}", dir.to_str().unwrap()))
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error retrieving CSS file"));

    css_request_mock.assert();
    Ok(())
}

#[test]
fn fails_on_malformed_css() -> TestResult {
    let server = MockServer::start();
    let css_request_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/css")
            .query_param("family", "Roboto:300,300i,400")
            .matches(|req| req.path == "/css");
        then.status(200)
            .body_from_file("tests/test_files/example_malformed.css");
    });

    let css_url = server.url("/css?family=Roboto:300,300i,400");

    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.arg(css_url);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("CSS Parsing Error"));

    // Check if server requests were called
    css_request_mock.assert();
    Ok(())
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

#[test]
fn downloads_font_files() -> TestResult {
    let server = MockServer::start();
    let css_request_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/css")
            .query_param("family", "Roboto:300,300i,400")
            .matches(|req| req.path == "/css");
        then.status(200)
            .body_from_file("tests/test_files/example.css");
    });

    let woff_i3x_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/ri3lx.woff2")
            .matches(|req| req.path == "/ri3lx.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--italic--300--latin-ext.woff2");
    });

    let woff_i3_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/ri3l.woff2")
            .matches(|req| req.path == "/ri3l.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--italic--300--latin.woff2");
    });

    let woff_n3x_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/rn3lx.woff2")
            .matches(|req| req.path == "/rn3lx.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--300--latin-ext.woff2");
    });

    let woff_n3_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/rn3l.woff2")
            .matches(|req| req.path == "/rn3l.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--300--latin.woff2");
    });

    let woff_n4x_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/rn4lx.woff2")
            .matches(|req| req.path == "/rn4lx.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--400--latin-ext.woff2");
    });

    let woff_n4_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/rn4l.woff2")
            .matches(|req| req.path == "/rn4l.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--400--latin.woff2");
    });

    let css_url = server.url("/css?family=Roboto:300,300i,400");
    let dir = Path::new("./tests/tmp");
    let working_dir = get_current_working_dir()?;

    // Remove the output directory if it already exists
    if dir.exists() && dir.starts_with(working_dir) {
        fs::remove_dir_all(dir)?;
    }

    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.arg(css_url)
        .arg(format!("--dir={}", dir.to_str().unwrap()))
        .arg("--font-url-prefix=/assets/fonts/")
        .assert()
        .success();

    // Check if server requests were called
    css_request_mock.assert();
    woff_i3x_mock.assert();
    woff_i3_mock.assert();
    woff_n3x_mock.assert();
    woff_n3_mock.assert();
    woff_n4x_mock.assert();
    woff_n4_mock.assert();

    // Check that the font files were downloaded correctly
    assert!(dir.join("ri3lx.woff2").exists());
    assert!(dir.join("ri3l.woff2").exists());
    assert!(dir.join("rn3lx.woff2").exists());
    assert!(dir.join("rn3l.woff2").exists());
    assert!(dir.join("rn4lx.woff2").exists());
    assert!(dir.join("rn4l.woff2").exists());

    // Rewritten css file
    let updated_css_file = dir.join("fonts.css");
    assert!(updated_css_file.exists());

    assert_eq!(
        file_contents(updated_css_file.to_str().unwrap())?,
        file_contents("tests/test_files/expected.css")?
    );

    Ok(())
}
