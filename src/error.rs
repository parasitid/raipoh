use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

use thiserror::Error;
use toml::de::Error as TomlError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("📁 IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("🔀 Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("🔤 Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("📝 TOML config error at {keys:?}: {message}\nℹ️ Problematic section: {}",
        raw.as_ref().map_or_else(|| "...".to_string(), |s| truncate_string(s, 150)))]
    Toml {
        message: String,
        keys: Vec<String>,
        raw: Option<String>,
    },

    #[error("🤖 Invalid LLM provider: {0}\nValid providers: Anthropic, OpenAI, OpenRouter, Ollama")]
    InvalidProvider(String),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Template error: {0}")]
    Template(#[from] handlebars::RenderError),

    #[error("Template registration error: {0}")]
    TemplateReg(#[from] handlebars::TemplateError),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    #[error("File too large: {0} bytes (max: {1} bytes)")]
    FileTooLarge(usize, usize),

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Generic error: {0}")]
    Generic(String),
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Self {
        Error::Toml {
            message: err.message().to_string(),
            keys: err.keys().into_iter().map(|k| k.to_string()).collect(),
            raw: err.raw().map(|s| s.to_string()),
        }
    }
}

impl From<rig::completion::CompletionError> for Error {
    fn from(err: rig::completion::CompletionError) -> Self {
        Error::Llm(err.to_string())
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        ))
    }
}

impl Error {
    pub fn is_file_not_found(&self) -> bool {
        matches!(self, Error::Io(ref e) if e.kind() == std::io::ErrorKind::NotFound)
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}
