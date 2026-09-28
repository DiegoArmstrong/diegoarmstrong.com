mod config;
mod content_manager;

use crate::config::SysPathConfig;
use std::path::PathBuf;

fn main() {
    let system_file_paths: &SysPathConfig = SysPathConfig::global();

    let home_page_path_source: &PathBuf = &system_file_paths.website_home_dir_source;
    let home_page_path_output: &PathBuf = &system_file_paths.website_home_dir_output;

    let blogs_to_publish_path_source: &PathBuf = &system_file_paths.markdown_content_dir_source;
    let blogs_to_publish_path_output: &PathBuf = &system_file_paths.markdown_content_dir_output;

    let photos_source: &PathBuf = &system_file_paths.pictures_dir_source;
    let photos_output: &PathBuf = &system_file_paths.pictures_dir_output;

    // Copy the photos from obsidian to docs/pictures/
    match content_manager::copy_photos(photos_source, photos_output) {
        Ok(()) => {
            println!("Photos Copied.");
        }
        Err(e) => {
            eprintln!("Photos failed to copy: {e:?}");
        }

    }

    // Render the blog posts.
    match content_manager::render_content(
        &blogs_to_publish_path_source,
        &blogs_to_publish_path_output,
    ) {
        Ok(()) => {
            println!("Blogs rendered.");
        }
        Err(e) => {
            eprintln!("Blogs failed to render: {e:?}");
        }
    }

    // Render the main page.
    match content_manager::render_content(&home_page_path_source, &home_page_path_output) {
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
