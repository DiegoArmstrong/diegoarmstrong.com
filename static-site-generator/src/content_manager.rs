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

    // Read the contents of the Markdown file.
    let markdown_bytes: Vec<u8> = fs::read(md_path)?;
    let markdown_string = String::from_utf8(markdown_bytes)
    .map_err(|_| ParserError::ReadFailure)?;

    // Create a new parser with all Markdown Options enabled.
    let parser = Parser::new_ext(&markdown_string, Options::all());

    // Store the HTML output of the parser into a new String.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Create a new file with the HTML content.
    fs::write(html_path, html_output)
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
            return Err(ParserError::WriteFailure);
        }
    }

    Ok(())
}

