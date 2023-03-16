pub(crate) use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author = "Wayne Duran <asartalo@gmail.com>",
    version = "0.1.0",
    about = "Download fonts for self-hosting",
    long_about = None
)]
pub(crate) struct Cli {
    pub css_path: String,

    #[arg(short = 'd', long = "dir", value_name = "DIRECTORY")]
    pub dir: Option<PathBuf>,

    #[arg(
        short = 'p',
        long = "font-url-prefix",
        value_name = "FONT_URL_PREFIX",
        default_value = ""
    )]
    pub font_url_prefix: String,
}
