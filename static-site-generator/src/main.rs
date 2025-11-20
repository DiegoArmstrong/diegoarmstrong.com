mod content_manager;
mod cli;
mod config;

fn main() {
    // Handle CLI arguments.
    cli::parse_args();
}
