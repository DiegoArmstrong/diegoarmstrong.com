use std::fs;
use std::io;
use std::path::Path;
use pulldown_cmark::{Parser, Options};

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Io(io::Error),
    ReadFailure,
    WriteFailure,
}

impl From<io::Error> for ParserError {
    fn from(e: io::Error) -> Self {
        ParserError::Io(e)
    }
}

fn html_file_starter() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title></title>
    <link rel="stylesheet" href="styles.css" />
</head>
<body>
"#.to_string()
}

fn html_file_end() -> String {
    "</body>\n</html>\n".to_string()
}

/// Create a new file.
pub fn create_file(path: &Path) -> Result<()> {

    // Create the new directory.
    fs::create_dir_all(path.parent().unwrap())
    .map_err(|_| ParserError::WriteFailure)?;

    // Write a small boiler plate.
    fs::write(path, "Start writing content!")
    .map_err(|_| ParserError::WriteFailure)?;

    Ok(())
}

/// Renders a new HTML file given markdown input.
pub fn render_file(md_path: &Path, html_path: &Path) -> Result<()> {

    // Read Markdown file
    let markdown_bytes: Vec<u8> = fs::read(md_path)?;
    let markdown_string = String::from_utf8(markdown_bytes)
        .map_err(|_| ParserError::ReadFailure)?;

    // Parse Markdown -> HTML body only
    let parser = Parser::new_ext(&markdown_string, Options::all());
    let mut html_body = String::new();
    pulldown_cmark::html::push_html(&mut html_body, parser);

    // Compose final HTML
    let mut final_html = String::new();
    final_html.push_str(&html_file_starter());
    final_html.push_str(&html_body);
    final_html.push_str(&html_file_end());

    // Write final HTML file
    fs::write(html_path, final_html)
        .map_err(|_| ParserError::WriteFailure)?;

    Ok(())
}

/// Render all of the content.
pub fn render_content(md_path: &Path, content_path: &Path) -> Result<()> {

    // Iterate thru each file in the Markdown path and render it.
    for entry in fs::read_dir(md_path)? {
        let entry = entry?;              
        let path = entry.path();        

        if path.is_file() {
            let stem = path
                .file_stem()              
                .and_then(|s| s.to_str()) 
                .ok_or(ParserError::WriteFailure)?; 

            let new_html_file = content_path.join(format!("{}.html", stem));

            render_file(&path, &new_html_file)?;

        } else if path.is_dir() {
            // don't do anything
        }
    }

    Ok(())
}

