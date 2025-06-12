use clap::{Args, Parser, Subcommand};
use raidme::{
    analyzer::RepoAnalyzer,
    config::Config,
    llm::{LlmBackend, LlmProvider},
    Result,
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
    /// Resume analysis from a previous checkpoint
    Resume(ResumeArgs),
    /// Show analysis status
    Status(StatusArgs),
}

#[derive(Args)]
struct AnalyzeArgs {
    /// Path to the repository to analyze
    #[arg(short, long)]
    repo_path: PathBuf,

    /// LLM provider to use (claude, openai, openrouter)
    #[arg(short, long, default_value = "claude")]
    provider: String,

    /// API key for the LLM provider
    #[arg(short, long)]
    api_key: Option<String>,

    /// Model to use (e.g., claude-3-sonnet, gpt-4, etc.)
    #[arg(short, long)]
    model: Option<String>,

    /// Output path for the knowledge file
    #[arg(short, long, default_value = "ai-knowledge.md")]
    output: PathBuf,

    /// Additional context or instructions for the AI
    #[arg(long)]
    context: Option<String>,

    /// Skip git commits for each step (useful for testing)
    #[arg(long)]
    no_commit: bool,
}

#[derive(Args)]
struct ResumeArgs {
    /// Path to the repository being analyzed
    #[arg(short, long)]
    repo_path: PathBuf,

    /// API key for the LLM provider
    #[arg(short, long)]
    api_key: Option<String>,
}

#[derive(Args)]
struct StatusArgs {
    /// Path to the repository
    #[arg(short, long)]
    repo_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze(args) => {
            let config = create_config(&args)?;
            let mut analyzer = RepoAnalyzer::new(config).await?;

            println!("🔍 Starting repository analysis...");
            println!("📁 Repo: {}", args.repo_path.display());
            println!("🤖 Provider: {}", args.provider);
            println!("📄 Output: {}", args.output.display());

            analyzer.analyze().await?;

            println!("✅ Analysis completed successfully!");
            println!("📄 Knowledge file generated: {}", args.output.display());
        }

        Commands::Resume(args) => {
            let config = Config::load_from_repo(&args.repo_path)?;
            let mut analyzer = RepoAnalyzer::from_checkpoint(config).await?;

            println!("🔄 Resuming analysis from checkpoint...");
            analyzer.resume().await?;

            println!("✅ Analysis resumed and completed!");
        }

        Commands::Status(args) => {
            let status = RepoAnalyzer::get_status(&args.repo_path)?;
            println!("📊 Analysis Status:");
            println!("{:#?}", status);
        }
    }

    Ok(())
}

fn create_config(args: &AnalyzeArgs) -> Result<Config> {
    let provider = match args.provider.as_str() {
        "claude" => LlmProvider::Claude,
        "openai" => LlmProvider::OpenAI,
        "openrouter" => LlmProvider::OpenRouter,
        _ => return Err("Invalid LLM provider. Use: claude, openai, or openrouter".into()),
    };

    let api_key = args.api_key.clone()
        .or_else(|| std::env::var("RAIDME_API_KEY").ok())
        .or_else(|| match provider {
            LlmProvider::Claude => std::env::var("CLAUDE_API_KEY").ok(),
            LlmProvider::OpenAI => std::env::var("OPENAI_API_KEY").ok(),
            LlmProvider::OpenRouter => std::env::var("OPENROUTER_API_KEY").ok(),
        })
        .ok_or("API key not provided. Use --api-key or set environment variable")?;

    let model = args.model.clone().unwrap_or_else(|| {
        match provider {
            LlmProvider::Claude => "claude-3-sonnet-20240229".to_string(),
            LlmProvider::OpenAI => "gpt-4-turbo-preview".to_string(),
            LlmProvider::OpenRouter => "anthropic/claude-3-sonnet".to_string(),
        }
    });

    Ok(Config {
        repo_path: args.repo_path.clone(),
        output_path: args.output.clone(),
        llm_provider: provider,
        api_key,
        model,
        context: args.context.clone(),
        commit_each_step: !args.no_commit,
    })
}
