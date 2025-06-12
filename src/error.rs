use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

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

impl From<rig_core::completion::CompletionError> for Error {
    fn from(err: rig_core::completion::CompletionError) -> Self {
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
