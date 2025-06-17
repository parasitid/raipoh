use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path,PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// LLM provider configuration
    pub llm: LlmConfig,

    /// Analysis configuration
    pub analysis: AnalysisConfig,

    /// Git configuration
    pub git: GitConfig,

    /// Template configuration
    pub template: TemplateConfig,

    /// Path of the output readme.ai.md
    pub output_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// The LLM provider to use
    pub provider: LlmProvider,

    /// API key for the provider
    pub api_key: String,

    /// Model name to use
    pub model: String,

    /// API base URL (for custom endpoints)
    pub base_url: Option<String>,

    /// Maximum retries
    pub max_retries: Option<u32>,

    /// Maximum tokens per request
    pub max_tokens: Option<u32>,

    /// Temperature for generation
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
    OpenRouter,
    Ollama,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: usize,

    /// File extensions to include in analysis
    pub include_extensions: Vec<String>,

    /// Directories to exclude from analysis
    pub exclude_dirs: Vec<String>,

    /// Files to exclude from analysisp
    pub exclude_files: Vec<String>,

    /// Maximum depth to traverse directories
    pub max_depth: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    /// Enable automatic git commits after each step
    pub auto_commit: bool,

    /// Git author name for commits
    pub author_name: String,

    /// Git author email for commits
    pub author_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Custom template directory
    pub template_dir: Option<PathBuf>,

    /// Output format
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    Json,
    Yaml,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: LlmConfig {
                provider: LlmProvider::Anthropic,
                api_key: String::new(),
                model: "claude-3-5-sonnet-20241022".to_string(),
                base_url: None,
                max_retries: Some(3),
                max_tokens: Some(4096),
                temperature: Some(0.7),
                output_path: "README.ai.md",
            },
            analysis: AnalysisConfig {
                max_file_size: 1024 * 1024, // 1MB
                include_extensions: vec![
                    "rs".to_string(),
                    "py".to_string(),
                    "js".to_string(),
                    "ts".to_string(),
                    "java".to_string(),
                    "cpp".to_string(),
                    "c".to_string(),
                    "h".to_string(),
                    "go".to_string(),
                    "md".to_string(),
                    "txt".to_string(),
                    "toml".to_string(),
                    "yaml".to_string(),
                    "yml".to_string(),
                    "json".to_string(),
                ],
                exclude_dirs: vec![
                    "target".to_string(),
                    "node_modules".to_string(),
                    ".git".to_string(),
                    "build".to_string(),
                    "dist".to_string(),
                    ".next".to_string(),
                    "__pycache__".to_string(),
                ],
                exclude_files: vec![
                    "package-lock.json".to_string(),
                    "Cargo.lock".to_string(),
                    "yarn.lock".to_string(),
                ],
                max_depth: Some(10),
            },
            git: GitConfig {
                auto_commit: true,
                author_name: "Raidme AI".to_string(),
                author_email: "raidme@ai.local".to_string(),
            },
            template: TemplateConfig {
                template_dir: None,
                output_format: OutputFormat::Markdown,
            },
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get the default configuration file path
    pub fn default_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| Error::ConfigError("Unable to determine config directory".to_string()))?;
        Ok(config_dir.join("raidme").join("config.toml"))
    }

    /// Load configuration from the default location or repo path. Fails if not found.
    pub fn load<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let repo_path = repo_path.as_ref();
        let repo_config_path = repo_path.join(".raidme.toml");

        if repo_config_path.exists() {
            return Self::from_file(&repo_config_path);
        }

        let config_path = Self::default_config_path()?;
        if config_path.exists() {
            return Self::from_file(&config_path);
        }

        Err(Error::ConfigError("No configuration file found".to_string()))
    }

    /// Load configuration if it exists, otherwise fall back to default
    pub fn load_or_default<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        match Self::load(repo_path) {
            Ok(config) => Ok(config),
            Err(e) if e.is_file_not_found() => Ok(Self::default()),
            Err(e) => Err(e),
        }
    }

    /// Store the configuration to the repo-local `.raidme.toml` file,
    /// excluding the API key from being saved.
    pub fn store<P: AsRef<Path>>(&self, repo_path: P) -> Result<()> {
        let repo_path = repo_path.as_ref();
        let mut clone = self.clone();
        clone.llm.api_key.clear(); // clear API key before saving

        let config_path = repo_path.join(".raidme.toml");
        clone.to_file(config_path)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.llm.api_key.is_empty() {
            return Err(Error::ConfigError("API key is required".to_string()));
        }

        if self.llm.model.is_empty() {
            return Err(Error::ConfigError("Model name is required".to_string()));
        }

        if self.analysis.max_file_size == 0 {
            return Err(Error::ConfigError("Max file size must be greater than 0".to_string()));
        }

        Ok(())
    }
}
