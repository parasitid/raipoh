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

use std::path::Path;
use tracing::info;

/// Main API for the raidme library
pub struct Raidme {
    config: Config,
    llm_client: LlmClient,
}

impl Raidme {
    /// Create a new Raidme instance with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let llm_client = LlmClient::new(&config)?;
        Ok(Self { config, llm_client })
    }

    /// Analyze a repository and generate knowledge file incrementally
    pub async fn analyze_repository<P: AsRef<Path>>(
        &self,
        repo_path: P,
        output_file: Option<&str>,
    ) -> Result<()> {
        let repo_path = repo_path.as_ref();
        info!("Starting analysis of repository: {}", repo_path.display());
        info!("Using config: {:?}", self.config);
        // let git_repo = GitRepository::open(repo_path)?;
        // let analyzer = RepositoryAnalyzer::new(repo_path, &self.config)?;
        // let generator = KnowledgeGenerator::new(&self.llm_client, &self.config);

        // Step 1: Analyze basic repository structure
        info!("Step 1: Analyzing basic repository structure");
        // let basic_info = analyzer.analyze_basic_structure().await?;
        // let mut knowledge = generator.generate_basic_knowledge(&basic_info).await?;

        // Commit the initial analysis
        // git_repo.commit_changes("Add basic repository analysis to knowledge file")?;

        // Step 2: Analyze README and root files
        info!("Step 2: Analyzing README and root files");
        // let readme_info = analyzer.analyze_readme_and_root().await?;
        // knowledge = generator.update_with_readme(&knowledge, &readme_info).await?;

        // git_repo.commit_changes("Add README and root files analysis")?;

        // Step 3: Analyze documentation
        info!("Step 3: Analyzing documentation");
        // let docs_info = analyzer.analyze_documentation().await?;
        // knowledge = generator.update_with_docs(&knowledge, &docs_info).await?;

        // git_repo.commit_changes("Add documentation analysis")?;

        // Step 4: Analyze package structure
        info!("Step 4: Analyzing package/directory structure");
        // let package_info = analyzer.analyze_package_structure().await?;
        // knowledge = generator.update_with_packages(&knowledge, &package_info).await?;

        // git_repo.commit_changes("Add package structure analysis")?;

        // Step 5: Final consolidation
        info!("Step 5: Final consolidation and cleanup");
        // let final_knowledge = generator.finalize_knowledge(&knowledge).await?;
        let final_knowledge = "dummy";

        // Write the final knowledge file
        let output_path = output_file.unwrap_or("README.ai.md");
        std::fs::write(repo_path.join(output_path), final_knowledge)?;

        // git_repo.commit_changes("Finalize AI knowledge file")?;

        info!("Repository analysis completed successfully");
        Ok(())
    }
}
