use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool};
use tokio::time::{sleep, Duration};

use crate::{
    config::{Config},
    llm::{LlmClient,LlmContext},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStep {
    pub id: String,
    pub step_type: StepType,
    pub status: StepStatus,
    pub input_data: String,
    pub output_data: Option<String>,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    Basic,
    Readme,
    Documentation,
    Package,
    Coding,
    Architecture,
    FinalConsolidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub id: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub title: String,
    pub content: String,
    pub relevance_score: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct RepositoryAnalyzer {
    config: Config,
    db: SqlitePool,
    llm_client: LlmClient,
    repo_path: PathBuf,
}

impl RepositoryAnalyzer {
    pub async fn new(config: Config, db: SqlitePool, llm_client: LlmClient, repo_path: PathBuf) -> Result<Self> {
        Ok(Self {
            config,
            db,
            llm_client,
            repo_path,
        })
    }

    pub async fn analyze(&self) -> Result<()> {
        println!("Starting repository analysis...");

        // Check if analysis is resuming or starting fresh
        let last_step = self.get_last_completed_step().await?;

        match last_step {
            None => {
                println!("Starting fresh analysis");
                self.run_full_analysis().await?;
            }
            Some(step) => {
                println!("Resuming analysis from step: {:?}", step.step_type);
                self.resume_analysis(step).await?;
            }
        }

        println!("Analysis completed successfully!");
        Ok(())
    }

    async fn run_full_analysis(&self) -> Result<()> {
        // Step 1: Gather basic information
        self.analyze_basic().await?;

        // Step 6: Generate final README.ai.md
        self.generate_final_consolidation().await?;

        Ok(())
    }

    async fn resume_analysis(&self, last_step: AnalysisStep) -> Result<()> {
        match last_step.step_type {
            StepType::Basic => {
                self.generate_final_readme().await?;
            }
            StepType::Readme => {
            }
            StepType::Documentation => {
            }
            StepType::Package => {
            }
            StepType::Coding => {
            }
            StepType::Architecture => {
            }
            StepType::FinalConsolidation => {
                self.generate_final_consolidation().await?;
                println!("Analysis already completed!");
            }
        }
        Ok(())
    }

    async fn analyze_basic(&self) -> Result<()> {
        println!("Analyzing basic repository information...");

        let step_id = uuid::Uuid::new_v4().to_string();
        self.create_analysis_step(&step_id, StepType::GlobalInfo, "Basic repository analysis").await?;

        let mut context = LlmContext::new(self.config.max_context_tokens);
        let analysis = self.llm_client.architecture_analysis(|| {
            let mut context = LlmContext::new(self.config.max_context_tokens);

            // Add package files with high priority
            for config_file in &["Cargo.toml", "package.json", "pyproject.toml"] {
                if let Ok(content) = std::fs::read_to_string(config_file) {
                    context.add_content_simple(content, 90, config_file.to_string());
                }
            }

            // Add directory structure with medium priority
            if let Ok(dir_structure) = self.get_directory_structure() {
                context.add_content_simple(dir_structure, 70, "Directory Structure".to_string());
            }

            // Add main source files with lower priority
            if let Ok(main_files) = self.get_main_source_files() {
                for (file_path, content) in main_files {
                    context.add_content_simple(content, 50, file_path);
                }
            }

            Ok(context)
        }).await?;


        // Store knowledge
        let knowledge_entry = KnowledgeEntry {
            id: uuid::Uuid::new_v4().to_string(),
            category: "basic".to_string(),
            subcategory: None,
            title: "Repository Basic Overview".to_string(),
            content: analysis.clone(),
            relevance_score: 1.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.store_knowledge_entry(&knowledge_entry).await?;
        self.complete_analysis_step(&step_id, &analysis).await?;

        println!("Basic analysis completed");
        Ok(())
    }

    // async fn analyze_documentation(&self) -> Result<()> {
    //     println!("Analyzing documentation...");

    //     let step_id = uuid::Uuid::new_v4().to_string();
    //     self.create_analysis_step(&step_id, StepType::Documentation, "Documentation analysis").await?;

    //     let docs_dirs = vec!["docs", "doc", "documentation", "wiki"];
    //     let mut docs_content = String::new();

    //     for docs_dir in docs_dirs {
    //         let docs_path = self.config.repository_path.join(docs_dir);
    //         if docs_path.exists() && docs_path.is_dir() {
    //             let content = self.read_documentation_recursive(&docs_path)?;
    //             docs_content.push_str(&format!("=== {} ===\n{}\n\n", docs_dir, content));
    //         }
    //     }

    //     if !docs_content.is_empty() {
    //         let current_knowledge = self.get_current_knowledge().await?;
    //         let prompt = self.create_documentation_analysis_prompt();
    //         let analysis = self.call_llm_with_retry(&prompt, &format!("{}\n\nExisting Knowledge:\n{}", docs_content, current_knowledge)).await?;

    //         let knowledge_entry = KnowledgeEntry {
    //             id: uuid::Uuid::new_v4().to_string(),
    //             category: "documentation".to_string(),
    //             subcategory: None,
    //             title: "Documentation Analysis".to_string(),
    //             content: analysis.clone(),
    //             relevance_score: 0.9,
    //             created_at: chrono::Utc::now(),
    //             updated_at: chrono::Utc::now(),
    //         };

    //         self.store_knowledge_entry(&knowledge_entry).await?;
    //         self.complete_analysis_step(&step_id, &analysis).await?;
    //     } else {
    //         self.complete_analysis_step(&step_id, "No documentation found").await?;
    //     }

    //     println!("Documentation analysis completed");
    //     Ok(())
    // }

    // async fn analyze_directory_structure(&self) -> Result<()> {
    //     println!("Analyzing directory structure...");

    //     let step_id = uuid::Uuid::new_v4().to_string();
    //     self.create_analysis_step(&step_id, StepType::DirectoryStructure, "Directory structure analysis").await?;

    //     let full_structure = self.get_directory_structure(&self.config.repository_path, 10)?;
    //     let current_knowledge = self.get_current_knowledge().await?;

    //     let prompt = self.create_directory_analysis_prompt();
    //     let analysis = self.call_llm_with_retry(&prompt, &format!("Directory Structure:\n{}\n\nExisting Knowledge:\n{}", full_structure, current_knowledge)).await?;

    //     let knowledge_entry = KnowledgeEntry {
    //         id: uuid::Uuid::new_v4().to_string(),
    //         category: "structure".to_string(),
    //         subcategory: None,
    //         title: "Directory Structure Analysis".to_string(),
    //         content: analysis.clone(),
    //         relevance_score: 0.8,
    //         created_at: chrono::Utc::now(),
    //         updated_at: chrono::Utc::now(),
    //     };

    //     self.store_knowledge_entry(&knowledge_entry).await?;
    //     self.complete_analysis_step(&step_id, &analysis).await?;

    //     println!("Directory structure analysis completed");
    //     Ok(())
    // }

    // async fn analyze_files(&self) -> Result<()> {
    //     println!("Analyzing key files...");

    //     let step_id = uuid::Uuid::new_v4().to_string();
    //     self.create_analysis_step(&step_id, StepType::FileAnalysis, "File analysis").await?;

    //     let key_files = self.identify_key_files()?;
    //     let current_knowledge = self.get_current_knowledge().await?;

    //     for file_path in key_files {
    //         if let Ok(content) = fs::read_to_string(&file_path) {
    //             // Skip very large files
    //             if content.len() > 50000 {
    //                 continue;
    //             }

    //             let relative_path = file_path.strip_prefix(&self.config.repository_path)
    //                 .unwrap_or(&file_path);

    //             let prompt = self.create_file_analysis_prompt(&relative_path.to_string_lossy());
    //             let analysis = self.call_llm_with_retry(&prompt, &format!("File Content:\n{}\n\nExisting Knowledge:\n{}", content, current_knowledge)).await?;

    //             let knowledge_entry = KnowledgeEntry {
    //                 id: uuid::Uuid::new_v4().to_string(),
    //                 category: "file".to_string(),
    //                 subcategory: Some(relative_path.to_string_lossy().to_string()),
    //                 title: format!("Analysis of {}", relative_path.to_string_lossy()),
    //                 content: analysis,
    //                 relevance_score: 0.7,
    //                 created_at: chrono::Utc::now(),
    //                 updated_at: chrono::Utc::now(),
    //             };

    //             self.store_knowledge_entry(&knowledge_entry).await?;
    //         }
    //     }

    //     self.complete_analysis_step(&step_id, "File analysis completed").await?;
    //     println!("File analysis completed");
    //     Ok(())
    // }

    // async fn generate_architecture_diagrams(&self) -> Result<()> {
    //     println!("Generating architecture diagrams...");

    //     let step_id = uuid::Uuid::new_v4().to_string();
    //     self.create_analysis_step(&step_id, StepType::ArchitectureDiagram, "Architecture diagram generation").await?;

    //     let current_knowledge = self.get_current_knowledge().await?;
    //     let prompt = self.create_architecture_prompt();
    //     let diagrams = self.call_llm_with_retry(&prompt, &current_knowledge).await?;

    //     let knowledge_entry = KnowledgeEntry {
    //         id: uuid::Uuid::new_v4().to_string(),
    //         category: "architecture".to_string(),
    //         subcategory: None,
    //         title: "Architecture Diagrams".to_string(),
    //         content: diagrams.clone(),
    //         relevance_score: 0.9,
    //         created_at: chrono::Utc::now(),
    //         updated_at: chrono::Utc::now(),
    //     };

    //     self.store_knowledge_entry(&knowledge_entry).await?;
    //     self.complete_analysis_step(&step_id, &diagrams).await?;

    //     println!("Architecture diagrams generated");
    //     Ok(())
    // }

    async fn generate_final_consolidation(&self) -> Result<()> {
        println!("Generating final README.ai.md...");

        let step_id = uuid::Uuid::new_v4().to_string();
        self.create_analysis_step(&step_id, StepType::FinalGeneration, "Final README generation").await?;



        let mut context = LlmContext::new(self.config.max_context_tokens);
        let consolidation = self.llm_client.final_consolidation(|| {
            let mut context = LlmContext::new(self.config.max_context_tokens);
            let all_knowledge = self.get_current_knowledge().await?;
            context.add_content_simple(all_knowledge, 90, config_file.to_string());
            Ok(context)
        }).await?;


        // Write to file
        fs::write(&self.config.output_path, &consolidation)
            .context("Failed to write README.ai.md")?;

        self.complete_analysis_step(&step_id, "README.ai.md generated successfully").await?;

        println!("Final README.ai.md generated at {:?}", self.config.output_path);
        Ok(())
    }
    // fn get_directory_structure(&self, path: &Path, max_depth: usize) -> Result<String> {
    //     let mut result = String::new();
    //     self.build_tree_string(path, &mut result, "", max_depth, 0)?;
    //     Ok(result)
    // }

    // fn build_tree_string(&self, path: &Path, result: &mut String, prefix: &str, max_depth: usize, current_depth: usize) -> Result<()> {
    //     if current_depth >= max_depth {
    //         return Ok(());
    //     }

    //     let mut entries: Vec<_> = fs::read_dir(path)?
    //         .filter_map(|entry| entry.ok())
    //         .filter(|entry| {
    //             if let Some(name) = entry.file_name().to_str() {
    //                 !self.should_ignore(name)
    //             } else {
    //                 false
    //             }
    //         })
    //         .collect();

    //     entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    //     for (i, entry) in entries.iter().enumerate() {
    //         let is_last = i == entries.len() - 1;
    //         let entry_prefix = if is_last { "└── " } else { "├── " };
    //         let next_prefix = if is_last { "    " } else { "│   " };

    //         result.push_str(&format!("{}{}{}\n", prefix, entry_prefix, entry.file_name().to_string_lossy()));

    //         if entry.path().is_dir() {
    //             self.build_tree_string(
    //                 &entry.path(),
    //                 result,
    //                 &format!("{}{}", prefix, next_prefix),
    //                 max_depth,
    //                 current_depth + 1
    //             )?;
    //         }
    //     }

    //     Ok(())
    // }

    // fn should_ignore(&self, name: &str) -> bool {
    //     self.config.ignore_patterns.iter().any(|pattern| {
    //         if pattern.contains('*') {
    //             // Simple glob matching
    //             let pattern = pattern.replace("*", "");
    //             name.contains(&pattern)
    //         } else {
    //             name == pattern
    //         }
    //     })
    // }

    // fn identify_key_files(&self) -> Result<Vec<PathBuf>> {
    //     let mut key_files = Vec::new();

    //     // Common important files
    //     let important_patterns = vec![
    //         "main.rs", "lib.rs", "mod.rs",
    //         "main.py", "__init__.py",
    //         "index.js", "app.js", "server.js",
    //         "Main.java", "Application.java",
    //         "main.go",
    //         "Dockerfile", "docker-compose.yml",
    //         "Makefile", "CMakeLists.txt",
    //     ];

    //     self.find_files_recursive(&self.config.repository_path, &important_patterns, &mut key_files)?;

    //     Ok(key_files)
    // }

    // fn find_files_recursive(&self, dir: &Path, patterns: &[&str], results: &mut Vec<PathBuf>) -> Result<()> {
    //     for entry in fs::read_dir(dir)? {
    //         let entry = entry?;
    //         let path = entry.path();

    //         if path.is_dir() {
    //             if !self.should_ignore(&entry.file_name().to_string_lossy()) {
    //                 self.find_files_recursive(&path, patterns, results)?;
    //             }
    //         } else if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
    //             if patterns.iter().any(|&pattern| filename.contains(pattern)) {
    //                 results.push(path);
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    // fn read_documentation_recursive(&self, dir: &Path) -> Result<String> {
    //     let mut content = String::new();

    //     for entry in fs::read_dir(dir)? {
    //         let entry = entry?;
    //         let path = entry.path();

    //         if path.is_file() {
    //             if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
    //                 if matches!(ext, "md" | "rst" | "txt" | "adoc") {
    //                     let file_content = fs::read_to_string(&path)?;
    //                     content.push_str(&format!("=== {} ===\n{}\n\n", path.file_name().unwrap().to_string_lossy(), file_content));
    //                 }
    //             }
    //         } else if path.is_dir() && !self.should_ignore(&entry.file_name().to_string_lossy()) {
    //             content.push_str(&self.read_documentation_recursive(&path)?);
    //         }
    //     }

    //     Ok(content)
    // }

    // Database operations

    async fn create_analysis_step(&self, id: &str, step_type: StepType, input_data: &str) -> Result<()> {
        let step_type_str = serde_json::to_string(&step_type)?;
        let status_str = serde_json::to_string(&StepStatus::InProgress)?;

        sqlx::query(
            "INSERT INTO analysis_steps (id, step_type, status, input_data, created_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(id)
        .bind(step_type_str)
        .bind(status_str)
        .bind(input_data)
        .bind(chrono::Utc::now())
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn complete_analysis_step(&self, id: &str, output_data: &str) -> Result<()> {
        let status_str = serde_json::to_string(&StepStatus::Completed)?;

        sqlx::query(
            "UPDATE analysis_steps SET status = $1, output_data = $2, completed_at = $3 WHERE id = $4"
        )
        .bind(status_str)
        .bind(output_data)
        .bind(chrono::Utc::now())
        .bind(id)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn get_last_completed_step(&self) -> Result<Option<AnalysisStep>> {
        let row = sqlx::query(
            "SELECT * FROM analysis_steps WHERE status = $1 ORDER BY created_at DESC LIMIT 1"
        )
        .bind(serde_json::to_string(&StepStatus::Completed)?)
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = row {
            Ok(Some(AnalysisStep {
                id: row.id,
                step_type: serde_json::from_str(&row.step_type)?,
                status: serde_json::from_str(&row.status)?,
                input_data: row.input_data,
                output_data: row.output_data,
                error_message: row.error_message,
                created_at: row.created_at,
                completed_at: row.completed_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn store_knowledge_entry(&self, entry: &KnowledgeEntry) -> Result<()> {
        sqlx::query(
            "INSERT INTO knowledge_entries (id, category, subcategory, title, content, relevance_score, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&entry.id)
        .bind(&entry.category)
        .bind(&entry.subcategory)
        .bind(&entry.title)
        .bind(&entry.content)
        .bind(entry.relevance_score)
        .bind(entry.created_at)
        .bind(entry.updated_at)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn get_current_knowledge(&self) -> Result<String> {
        let rows = sqlx::query(
            "SELECT category, title, content FROM knowledge_entries ORDER BY relevance_score DESC, created_at ASC"
        )
        .fetch_all(&self.db)
        .await?;

        let mut knowledge = String::new();
        for row in rows {
            knowledge.push_str(&format!("## {} - {}\n{}\n\n", row.category, row.title, row.content));
        }

        Ok(knowledge)
    }

    // Prompt creation methods

    fn create_global_analysis_prompt(&self) -> String {
        r#"You are an expert software architect analyzing a git repository. Your task is to provide a comprehensive overview of the repository based on the provided information.

Please analyze and provide:

1. **Project Purpose**: What does this project do? What problem does it solve?
2. **Technology Stack**: What languages, frameworks, and tools are used?
3. **Architecture Overview**: High-level architecture and design patterns
4. **Key Components**: Main modules, services, or components
5. **Dependencies**: Important external dependencies and their purposes
6. **Build/Deployment**: How is the project built and deployed?

Focus on information that would help an AI coding assistant understand the project structure and make better code suggestions. Be concise but comprehensive."#.to_string()
    }

    fn create_documentation_analysis_prompt(&self) -> String {
        r#"You are analyzing project documentation to extract architectural and implementation details.

Based on the documentation provided, please identify and extract:

1. **Architectural Decisions**: Design patterns, architectural styles, key decisions
2. **API Specifications**: Endpoints, interfaces, contracts
3. **Configuration**: Important configuration options and their purposes
4. **Workflows**: Key processes, business logic flows
5. **Integration Points**: External systems, services, databases
6. **Development Guidelines**: Coding standards, best practices mentioned

Combine this with the existing knowledge to create a more complete understanding. Avoid duplicating information already well-covered in the existing knowledge."#.to_string()
    }

    fn create_directory_analysis_prompt(&self) -> String {
        r#"You are analyzing the directory structure of a software project to understand its organization and architecture.

Based on the directory structure provided, please analyze:

1. **Project Organization**: How is the code organized? What patterns are used?
2. **Module Boundaries**: How are different concerns separated?
3. **Layer Architecture**: Are there clear layers (presentation, business, data)?
4. **Package Structure**: What do the directory names suggest about functionality?
5. **Configuration and Assets**: Where are configs, resources, and static files?
6. **Testing Structure**: How are tests organized?

Focus on insights that help understand the codebase structure and inform better development decisions."#.to_string()
    }

    fn create_file_analysis_prompt(&self, file_path: &str) -> String {
        format!(r#"You are analyzing a specific source code file to understand its role in the project architecture.

File: {}

Please analyze this file and provide:

1. **Purpose**: What is this file's main responsibility?
2. **Key Functions/Classes**: Main components and their roles
3. **Dependencies**: What other parts of the system does it depend on?
4. **Interfaces**: What APIs, contracts, or interfaces does it define/implement?
5. **Patterns**: What design patterns or architectural patterns are used?
6. **Integration Points**: How does it connect to other system components?

Focus on architectural insights rather than implementation details. Consider how this file fits into the broader system design."#, file_path)
    }

    fn create_architecture_prompt(&self) -> String {
        r#"You are creating architecture diagrams and documentation based on your analysis of a software project.

Based on all the knowledge gathered, please create:

1. **System Architecture Diagram**: A high-level view of major components and their relationships (use ASCII art or Mermaid syntax)
2. **Component Interaction Flow**: How do the main components communicate?
3. **Data Flow**: How does data move through the system?
4. **Deployment Architecture**: How is the system deployed and what are the runtime components?
5. **Technology Stack Diagram**: Visual representation of the technology layers

Use Mermaid diagram syntax where possible for clear, readable diagrams. Focus on helping developers understand the system's architecture quickly."#.to_string()
    }

    fn create_final_readme_prompt(&self) -> String {
        r#"You are creating a comprehensive README.ai.md file that will serve as an architecture knowledge base for AI coding assistants.

Based on all the analyzed knowledge, create a well-structured README.ai.md that includes:

# Structure Requirements:
1. **Overview**: Project purpose and key capabilities
2. **Architecture**: High-level system design and patterns
3. **Project Structure**: Directory layout and organization
4. **Key Components**: Major modules and their responsibilities
5. **Technology Stack**: Languages, frameworks, tools used
6. **APIs and Interfaces**: Key contracts and endpoints
7. **Data Models**: Important data structures and schemas
8. **Configuration**: Key configuration options and their purposes
9. **Development Workflow**: Build, test, deploy processes
10. **Integration Points**: External dependencies and services
11. **Diagrams**: Architecture and flow diagrams

# Format Requirements:
- Use clear Markdown formatting
- Include code examples where relevant
- Use Mermaid diagrams for visual representations
- Structure information hierarchically
- Focus on information that helps AI assistants make better code suggestions
- Be comprehensive but avoid unnecessary verbosity

The goal is to create documentation that enables any AI coding assistant to understand the project deeply and provide contextually appropriate suggestions and modifications."#.to_string()
    }
}
