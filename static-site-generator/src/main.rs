mod cli;
mod config;
mod content_manager;

fn main() {
    // Handle CLI arguments.
    cli::parse_args();
}
