use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;
use tinytemplate::TinyTemplate;
use dotenvy::dotenv;

/// System Path Configuration: local file system paths not to be shared
#[derive(Debug)]
pub struct SysPathConfig {
    pub website_home_dir_source: PathBuf,
    pub website_home_dir_output: PathBuf,
    pub markdown_content_dir_source: PathBuf,
    pub markdown_content_dir_output: PathBuf,
    pub template_css_dir: PathBuf,
    pub aesthetics_dir: PathBuf,
    pub styles_css_dir_output: PathBuf,
    pub pictures_dir_source: PathBuf,
    pub pictures_dir_output: PathBuf,
}

// global static storage for the initialized struct
static SYSPATH_CONFIG: OnceLock<SysPathConfig> = OnceLock::new();

impl SysPathConfig {
    /// Initializes and fetches the global instance from environment variables
    pub fn global() -> &'static Self {

        match dotenvy::from_filename(".env.syspaths") {
            Ok(path) => eprintln!("loaded env file from {path:?}"),
            Err(e) => eprintln!("no env file found: {e}"),
        }

        SYSPATH_CONFIG.get_or_init(|| {
            // Obsidian Markdown system home directory: index.html + styles.css
            let website_home_dir_source_raw =
                env::var("WEBSITE_HOME_DIRECTORY_SOURCE").unwrap_or_else(|_| "./data".to_string());

            // Github repo deployed content directory: home page
            let website_home_dir_output_raw =
                env::var("WEBSITE_HOME_DIRECTORY_OUTPUT").unwrap_or_else(|_| "./data".to_string());

            // Obsidian Markdown system blogs directory
            let markdown_content_dir_source_raw =
                env::var("PUBLISH_MARKDOWN_CONTENT_DIRECTORY_SOURCE")
                    .unwrap_or_else(|_| "./data".to_string());

            // Github repo deployed content directory: blogs
            let markdown_content_dir_output_raw =
                env::var("PUBLISH_MARKDOWN_CONTENT_DIRECTORY_OUTPUT")
                    .unwrap_or_else(|_| "./data".to_string());

            // Template CSS Directory
            let template_css_dir_raw =
                env::var("TEMPLATE_CSS_DIR")
                    .unwrap_or_else(|_| "./data".to_string());

            // Aesthetics.toml Directory
            let aesthetics_dir_raw =
                env::var("AESTHETICS_DIR")
                    .unwrap_or_else(|_| "./data".to_string());

            // Styles.css Github deployed directory
            let styles_css_dir_output_raw =
                env::var("STYLES_CSS_DIR_OUTPUT")
                    .unwrap_or_else(|_| "./data".to_string());

            // Pictures Source Directory
            let pictures_source_dir_raw =
                env::var("PICTURES_DIR_SOURCE")
                    .unwrap_or_else(|_| "./data".to_string());

            // Pictures Output Directory
            let pictures_output_dir_raw =
                env::var("PICTURES_DIR_OUTPUT")
                    .unwrap_or_else(|_| "./data".to_string());

            Self {
                website_home_dir_source: PathBuf::from(website_home_dir_source_raw),
                website_home_dir_output: PathBuf::from(website_home_dir_output_raw),
                markdown_content_dir_source: PathBuf::from(markdown_content_dir_source_raw),
                markdown_content_dir_output: PathBuf::from(markdown_content_dir_output_raw),
                template_css_dir: PathBuf::from(template_css_dir_raw),
                aesthetics_dir: PathBuf::from(aesthetics_dir_raw),
                styles_css_dir_output: PathBuf::from(styles_css_dir_output_raw),
                pictures_dir_source: PathBuf::from(pictures_source_dir_raw),
                pictures_dir_output: PathBuf::from(pictures_output_dir_raw),
            }
        })
    }
}

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
    heading3_colour: String,
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
    let toml_text = fs::read_to_string(path).map_err(|_| ConfigError::LoadError)?;

    let cfg: ThemeConfig = toml::from_str(&toml_text).map_err(|_| ConfigError::LoadError)?;

    Ok(cfg)
}

fn render_css(theme: &ThemeConfig) -> Result<String, ConfigError> {

    let system_file_paths: &SysPathConfig = SysPathConfig::global();

    let template_css_dir: &PathBuf = &system_file_paths.template_css_dir;

    // Load template from a file
    let template_src =
        fs::read_to_string(template_css_dir)
            .map_err(|_| ConfigError::TemplateError)?;

    let mut tt = TinyTemplate::new();
    tt.add_template("css", &template_src)
        .map_err(|_| ConfigError::TemplateError)?;

    // Render using ThemeConfig as the context
    let css = tt
        .render("css", theme)
        .map_err(|_| ConfigError::TemplateError)?;

    Ok(css)
}

pub fn build_config() -> Result<(), ConfigError> {
    let system_file_paths: &SysPathConfig = SysPathConfig::global();

    let aesthetics_dir: &PathBuf = &system_file_paths.aesthetics_dir;
    let styles_css_dir: &PathBuf = &system_file_paths.styles_css_dir_output;

    let theme: ThemeConfig =
        load_theme(aesthetics_dir.to_str().unwrap())?;
    let css = render_css(&theme)?;

    fs::write(styles_css_dir, css)
        .map_err(|_| ConfigError::BuildError)?;

    Ok(())
}
