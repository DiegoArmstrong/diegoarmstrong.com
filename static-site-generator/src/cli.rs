use crate::content_manager;
use slugify::slugify;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "myssg", version, about = "Tiny static site generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new blog post markdown file
    NewPost {
        /// Title of the blog post
        title: String,

        /// Directory for blog posts
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/markdown/blog/")]
        dir: PathBuf,
    },

    /// Build the site (markdown -> HTML)
    Build {
        /// Input content directory
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/markdown/blog/")]
        input: PathBuf,

        /// Output directory for generated HTML
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/content/blog/")]
        output: PathBuf,
    },
}

/// Parse the inputted CLI arguments.
pub fn parse_args() {
    let cli = Cli::parse();

    match cli.command {

        // Handle the NewPost command.
        Commands::NewPost { title, dir } => {
            // Turn title into a slug: "title-of-new-post"
            let slug = slugify!(&title);

            // Build path: content/blog/title-of-new-post.md
            let path = dir.join(format!("{slug}.md"));

            // Create the blog post file. 
            match content_manager::create_file(&path) {
                Ok(()) => {
                    println!("New file created.");
                }
                Err(e) => {
                    eprintln!("New file failed to be created: {e:?}");
                }
            }
        }

        // Handle the Build Command.
        Commands::Build { input, output } => {

            match content_manager::render_content(&input, &output) {
                Ok(()) => {
                    println!("Content rendered.");
                }
                Err(e) => {
                    eprintln!("Content failed to render: {e:?}");
                }
            }

        }
    }
}


