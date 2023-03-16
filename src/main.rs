mod command;
mod http;
mod read;
mod result;

fn main() {
    if let Err(e) = command::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
