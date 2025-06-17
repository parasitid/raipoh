use clap::{Args, Parser, Subcommand};
use raidme::{
    analyzer::RepositoryAnalyzer,
    config::{Config,LlmProvider},
    Raidme,
    Error,
    LlmClient,
    Result
};
use std::path::PathBuf;
use tokio;

#[derive(Parser)]
#[command(name = "raidme")]
#[command(about = "AI-powered repository analyzer that generates knowledge documentation")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a repository and generate knowledge documentation
    Analyze(AnalyzeArgs),

    /// Show analysis status
    Status(StatusArgs),
}

#[derive(Args)]
struct AnalyzeArgs {
    /// Path to the repository to analyze
    #[arg(short, long)]
    repo_path: PathBuf,

    /// LLM provider to use (anthropic, openai, openrouter)
    #[arg(short, long)]
    provider: Option<String>,

    /// API key for the LLM provider
    #[arg(short, long)]
    api_key: Option<String>,

    /// Model to use (e.g., anthropic-3-sonnet, gpt-4, etc.)
    #[arg(short, long)]
    model: Option<String>,

    /// Custom base URL for LLM API (overrides default)
    #[arg(long)]
    base_url: Option<String>,

    /// Output path for the knowledge file
    #[arg(short, long, default_value = "README.ai.md")]
    output: PathBuf,

    /// Additional context or instructions for the AI
    #[arg(long)]
    context: Option<String>,

    /// Skip git commits for each step (useful for testing)
    #[arg(long)]
    no_commit: bool,
}

#[derive(Args)]
struct StatusArgs {
    /// Path to the repository
    #[arg(short, long)]
    repo_path: PathBuf,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{}", err); // <- This uses your #[error("...")] message!
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze(args) => {
            let config = create_config(&args)?;

            let raidme = Raidme::new(repo_path, config).await?;
            // Set up database connection

            println!("🔍 Starting repository analysis...");
            println!("📁 Repo: {}", args.repo_path.display());
            println!("🤖 Provider: {}", args.provider.as_deref().unwrap_or("default"));
            println!("📄 Output: {}", args.output.display());
            let llm_client = LlmClient::new(&config)?;
            let analyzer = RepositoryAnalyzer::new(config, db, llm_client, &args.repo_path)?;

            analyzer.analyze().await?;

            println!("✅ Analysis completed successfully!");
            println!("📄 Knowledge file generated: {}", args.output.display());
        }

        Commands::Status(args) => {
            let config = Config::load(&args.repo_path)?;
            println!("Using config: {:?}", config);
            // let status = RepoAnalyzer::get_status(&args.repo_path)?;
            println!("📊 Analysis Status:");
            // println!("{:#?}", status);
            println!("dummy");
        }
    }

    Ok(())
}

fn create_config(args: &AnalyzeArgs) -> Result<Config> {
    // Load the base config from repo or global file (or default)
    let mut config = Config::load_or_default(&args.repo_path)?;

    // Override LLM provider if passed in CLI args
    if let Some(provider) = &args.provider {
        config.llm.provider = match provider.as_str() {
            "anthropic" => LlmProvider::Anthropic,
            "openai" => LlmProvider::OpenAI,
            "openrouter" => LlmProvider::OpenRouter,
            _ => return Err(Error::InvalidProvider(provider.clone())),
        };
    }

    // Override api_key with CLI or env vars or keep existing
    config.llm.api_key = args.api_key.clone()
        .or_else(|| std::env::var("RAIDME_API_KEY").ok())
        .or_else(|| match config.llm.provider {
            LlmProvider::Anthropic => std::env::var("ANTHROPIC_API_KEY").ok(),
            LlmProvider::OpenAI => std::env::var("OPENAI_API_KEY").ok(),
            LlmProvider::OpenRouter => std::env::var("OPENROUTER_API_KEY").ok(),
            LlmProvider::Ollama => None,
        })
        .unwrap_or_else(|| config.llm.api_key.clone());

    // Override base URL if specified
    if let Some(base_url) = &args.base_url {
        config.llm.base_url = Some(base_url.clone());
    }

    // Determine the model with the following precedence:
    // 1. CLI argument
    // 2. Config value (non-empty)
    // 3. Provider default

    config.llm.model = args.model.clone()
        .filter(|m| !m.is_empty())
        .or_else(|| {
            if !config.llm.model.is_empty() {
                Some(config.llm.model.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| match config.llm.provider {
            LlmProvider::Anthropic => "claude-3-sonnet-20240229".to_string(),
            LlmProvider::OpenAI => "gpt-4-turbo-preview".to_string(),
            LlmProvider::OpenRouter => "anthropic/claude-3-sonnet".to_string(),
            LlmProvider::Ollama => "ollama-default".to_string(),
        });
    
    // You can override other parts similarly, e.g. context, commit_each_step, etc.

    config.validate()?;

    // Store the config (excluding API key)
    config.store(&args.repo_path)?;

    Ok(config)
}
