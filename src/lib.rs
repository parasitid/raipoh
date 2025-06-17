mod analyzer;
pub mod config;
pub mod error;
// pub mod generator;
// pub mod git;
pub mod llm;
// pub mod template;

pub use analyzer::RepositoryAnalyzer;
pub use config::{Config, LlmProvider};
pub use error::{Error, Result};
// pub use generator::KnowledgeGenerator;
// pub use git::GitRepository;
pub use llm::LlmClient;

use std::path::{PathBuf};
use sqlx::{sqlite::SqlitePool, migrate::Migrator};
//
/// Main API for the raidme library
pub struct Raidme {
    config: Config,
    llm_client: LlmClient,
    db: SqlitePool,
}

static MIGRATOR: Migrator = sqlx::migrate!(); // <- macro looks for ./migrations/

async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    MIGRATOR.run(pool).await.map_err(Error::Migrate)
}

impl Raidme {
    /// Create a new Raidme instance with the given configuration
    pub async fn new(repo_path: PathBuf, config: Config) -> Result<Self> {
        // // Initialize database connection
            // Set up database connection
            let database_path = format!("{}/.raidme.db", repo_path.display());
            let database_url = format!("sqlite:{}", database_path);
            let db = SqlitePool::connect(&database_url)
                .await
                .map_err(Error::Sqlx)?;


            // Verify or create tables using migration
           run_migrations(&db).await?;
            println!("Created config: {:?}", config);
            println!("Database: {}", database_path);

            // Validate config before saving
            config.validate()?;

            // Store the config (excluding API key)
            config.store(&repo_path)?;

            let llm_client = LlmClient::new(&config)?;
            Ok(Self {
                    config,
                    llm_client,
                    db
            })
    }

    // /// Analyze a repository and generate knowledge file incrementally
    // pub async fn analyze_repository<P: AsRef<Path>>(
    //     &self,
    //     repo_path: P,
    //     output_file: Option<&str>,
    // ) -> Result<()> {
    //     Ok(())
    // }
}
