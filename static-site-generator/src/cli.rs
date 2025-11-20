use crate::content_manager;
use crate::config;
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
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/content/markdown/blog/")]
        dir: PathBuf,
    },

    /// Build the site (markdown -> HTML)
    Build {
        /// Input blog directory (markdown)
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/content/markdown/blog/")]
        blog_input: PathBuf,

        /// Output blog directory (HTML)
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/content/html/blog/")]
        blog_output: PathBuf,

        /// Input blog directory (markdown)
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/content/markdown/main-page/")]
        main_page_input: PathBuf,

        /// Output blog directory (HTML)
        #[arg(long, default_value = "/home/diego/dev/diegoarmstrong.com/content/html/main-page/")]
        main_page_output: PathBuf,

    },
}

/// Handle a new-post command
fn handle_new_post(title: String, dir: PathBuf) {

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

/// Handle a build command
fn handle_build(blog_input: PathBuf, blog_output: PathBuf, main_page_input: PathBuf, main_page_output: PathBuf) {

    // Render the blog posts. 
    match content_manager::render_content(&blog_input, &blog_output) {
        Ok(()) => {
            println!("Blogs rendered.");
        }
        Err(e) => {
            eprintln!("Blogs failed to render: {e:?}");
        }
    }

    // Render the main page. 
    match content_manager::render_content(&main_page_input, &main_page_output) {
        Ok(()) => {
            println!("Main Page rendered.");
        }
        Err(e) => {
            eprintln!("Main Page failed to render: {e:?}");
        }
    }

    // Render the CSS.
    match config::build_config() {
        Ok(()) => {
            println!("CSS rendered.");
        }
        Err(e) => {
            eprintln!("CSS failed to render: {e:?}");
        }
    }
}

/// Parse the inputted CLI arguments.
pub fn parse_args() {

    let cli = Cli::parse();

    match cli.command {

        // Handle the NewPost command.
        Commands::NewPost { title, dir } => {
            handle_new_post(title, dir);
        }

        // Handle the Build Command.
        Commands::Build { blog_input, blog_output, main_page_input, main_page_output} => {
            handle_build(blog_input, blog_output, main_page_input, main_page_output);
        }
    }
}


