use serde::Serialize;
use tinytemplate::TinyTemplate;
use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    LoadError,
    TemplateError,
    BuildError,
}

#[derive(Deserialize, Serialize)]
struct BaseColours {
    background_colour: String,
    foreground_colour: String,
}

#[derive(Deserialize, Serialize)]
struct HeadingColours {
    heading1_colour: String,
    heading2_colour: String,
}

#[derive(Deserialize, Serialize)]
struct LinkColours {
    link_colour: String,
    link_hover_colour: String,
}

#[derive(Deserialize, Serialize)]
struct CodeBlockColours {
    code_background_colour: String,
    code_text_colour: String,
}

#[derive(Deserialize, Serialize)]
struct AccentColours {
    accent_colour: String,
    error_colour: String,
    warning_colour: String,
}

#[derive(Deserialize, Serialize)]
struct FontSizes {
    heading1_size: String,
    heading2_size: String,
    heading3_size: String,
    body_size: String,
}

#[derive(Deserialize, Serialize)]
struct Font {
    font_family: String,
    font_sizes: FontSizes,
}

#[derive(Deserialize, Serialize)]
struct ThemeConfig {
    base_colours: BaseColours,
    heading_colours: HeadingColours,
    link_colours: LinkColours,
    code_block_colours: CodeBlockColours,
    accent_colours: AccentColours,
    font: Font,
}

fn load_theme(path: &str) -> Result<ThemeConfig, ConfigError> {
    let toml_text = fs::read_to_string(path)
    .map_err(|_| ConfigError::LoadError)?;

    let cfg: ThemeConfig = toml::from_str(&toml_text)
    .map_err(|_| ConfigError::LoadError)?;

    Ok(cfg)
}

fn render_css(theme: &ThemeConfig) -> Result<String, ConfigError> {
    // Load template from a file
    let template_src = fs::read_to_string("/home/diego/dev/diegoarmstrong.com/config/styles-template.css")
    .map_err(|_| ConfigError::TemplateError)?;

    let mut tt = TinyTemplate::new();
    tt.add_template("css", &template_src)
    .map_err(|_| ConfigError::TemplateError)?;

    // Render using ThemeConfig as the context
    let css = tt.render("css", theme)
    .map_err(|_| ConfigError::TemplateError)?;

    Ok(css)
}

pub fn build_config() -> Result<(), ConfigError> {

    let theme: ThemeConfig = load_theme("/home/diego/dev/diegoarmstrong.com/config/aesthetics.toml")?;
    let css = render_css(&theme)?;

    fs::write("/home/diego/dev/diegoarmstrong.com/content/styles.css", css)
    .map_err(|_| ConfigError::BuildError)?;

    Ok(())
}





