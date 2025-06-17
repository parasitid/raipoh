use crate::config::{Config, LlmProvider};
use crate::error::Result as ResultOrErr;
use std::time::Duration;
use tokio::time::sleep;
use std::collections::VecDeque;
use anyhow::{Result, Context};


#[derive(Debug, Clone)]
pub struct ContentItem {
    pub content: String,
    pub priority: u32, // Higher number = higher priority
    pub title: String,
    pub can_summarize: bool, // Whether this content can be summarized if needed
}

impl ContentItem {
    pub fn new(content: String, priority: u32, title: String) -> Self {
        Self {
            content,
            priority,
            title,
            can_summarize: true,
        }
    }

    pub fn new_non_summarizable(content: String, priority: u32, title: String) -> Self {
        Self {
            content,
            priority,
            title,
            can_summarize: false,
        }
    }

    pub fn estimated_tokens(&self) -> usize {
        // Rough estimation: ~4 characters per token
        self.content.len() / 4
    }
}

#[derive(Debug)]
pub struct LlmContext {
    pub items: Vec<ContentItem>,
    pub max_context_tokens: usize,
}

impl LlmContext {
    pub fn new(max_context_tokens: usize) -> Self {
        Self {
            items: Vec::new(),
            max_context_tokens,
        }
    }

    pub fn add_content(&mut self, item: ContentItem) {
        self.items.push(item);
    }

    pub fn add_content_simple(&mut self, content: String, priority: u32, title: String) {
        self.add_content(ContentItem::new(content, priority, title));
    }

    pub fn total_estimated_tokens(&self) -> usize {
        self.items.iter().map(|item| item.estimated_tokens()).sum()
    }

    // Sort items by priority (highest first)
    fn sort_by_priority(&mut self) {
        self.items.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    // Create a context string that fits within the token limit
    pub async fn build_context(&mut self, llm_client: &dyn LlmClient) -> Result<String> {
        self.sort_by_priority();

        let total_tokens = self.total_estimated_tokens();
        if total_tokens <= self.max_context_tokens {
            // Everything fits, return as-is
            return Ok(self.items.iter()
                .map(|item| format!("=== {} ===\n{}\n\n", item.title, item.content))
                .collect::<Vec<_>>()
                .join(""));
        }

        // Need to reduce context size
        let mut result_items = Vec::new();
        let mut remaining_tokens = self.max_context_tokens;

        for item in &mut self.items {
            let item_tokens = item.estimated_tokens();

            if item_tokens <= remaining_tokens {
                // Item fits as-is
                result_items.push(format!("=== {} ===\n{}\n\n", item.title, item.content));
                remaining_tokens -= item_tokens;
            } else if item.can_summarize && remaining_tokens > 100 {
                // Try to summarize the item to fit
                let target_length = (remaining_tokens - 50) * 4; // Convert tokens back to approximate chars
                let summarized = self.summarize_content(llm_client, &item.content, &item.title, target_length).await?;
                let summarized_tokens = summarized.len() / 4;

                if summarized_tokens <= remaining_tokens {
                    result_items.push(format!("=== {} (Summarized) ===\n{}\n\n", item.title, summarized));
                    remaining_tokens -= summarized_tokens;
                } else {
                    // Even summarized version doesn't fit, skip this item
                    println!("Warning: Skipping '{}' - too large even when summarized", item.title);
                }
            } else {
                // Item doesn't fit and can't be summarized, skip it
                println!("Warning: Skipping '{}' - exceeds remaining context space", item.title);
            }

            if remaining_tokens < 100 {
                // Not enough space for more content
                break;
            }
        }

        Ok(result_items.join(""))
    }

    async fn summarize_content(&self, llm_client: &dyn LlmClient, content: &str, title: &str, target_length: usize) -> Result<String> {
        let summarize_prompt = format!(
            "Please provide a concise summary of the following content from '{}'. \
            The summary should be approximately {} characters long and capture the key information:\n\n{}",
            title, target_length, content
        );

        // Use a simple context for summarization
        llm_client.generate_completion(&summarize_prompt, "").await
    }
}


pub trait Agent {
    async fn prompt(&self, prompt: &str) -> crate::Result<String>;
}

use rig::completion::Prompt;
use rig::providers::{anthropic, openai, ollama, openrouter};

/// Unified LLM client that abstracts over different providers
pub struct LlmClient {
    pub basic_analysis_agent: Box<dyn Agent + Send + Sync>,
    pub readme_analysis_agent: Box<dyn Agent + Send + Sync>,
    pub documentation_analysis_agent: Box<dyn Agent + Send + Sync>,
    pub coding_analysis_agent: Box<dyn Agent + Send + Sync>,
    pub architecture_analysis_agent: Box<dyn Agent + Send + Sync>,
    pub package_analysis_agent: Box<dyn Agent + Send + Sync>,
    pub file_analysis_agent: Box<dyn Agent + Send + Sync>,
    pub final_consolidation_agent: Box<dyn Agent + Send + Sync>,
    pub summarization_agent: Box<dyn Agent + Send + Sync>,
    pub provider: LlmProvider,
    pub max_retries: u32,
    pub retry_delay_seconds: u32,
}

impl LlmClient {
    /// Create a new LLM client from configuration
    pub fn new(config: &Config) -> ResultOrErr<Self> {
        config.validate()?;

        // Create base client based on provider
        let (basic_agent, file_agent, readme_agent, doc_agent, package_agent, coding_agent, architecture_agent, final_agent, summarization_agent) = match config.llm.provider {
            LlmProvider::OpenAI => {
                let mut client = openai::Client::new(&config.llm.api_key);
                if let Some(base_url) = &config.llm.base_url {
                    client = client.with_base_url(base_url);
                }

                let basic = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::basic_analysis())
                    .build();
                let file = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::file_analysis())
                    .build();
                let readme = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::readme_analysis())
                    .build();
                let doc = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::documentation_analysis())
                    .build();
                let package = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::package_analysis())
                    .build();
                let coding = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::coding_analysis())
                    .build();
                let architecture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::architecture_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();
                let summarization = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::summarization())
                    .build();
                (Box::new(basic) as Box<dyn Agent + Send + Sync>,
                 Box::new(file) as Box<dyn Agent + Send + Sync>,
                 Box::new(readme) as Box<dyn Agent + Send + Sync>,
                 Box::new(doc) as Box<dyn Agent + Send + Sync>,
                 Box::new(package) as Box<dyn Agent + Send + Sync>,
                 Box::new(coding) as Box<dyn Agent + Send + Sync>,
                 Box::new(architecture) as Box<dyn Agent + Send + Sync>,
                 Box::new(final_agent) as Box<dyn Agent + Send + Sync>,
                 Box::new(summarization) as Box<dyn Agent + Send + Sync>)
            }
            LlmProvider::Anthropic => {
                let client = anthropic::Client::new(&config.llm.api_key);

                let basic = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::basic_analysis())
                    .build();
                let file = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::file_analysis())
                    .build();
                let readme = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::readme_analysis())
                    .build();
                let doc = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::documentation_analysis())
                    .build();
                let coding = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::coding_analysis())
                    .build();
                let architecture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::architecture_analysis())
                    .build();
                let package = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::package_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();

                (Box::new(basic) as Box<dyn Agent + Send + Sync>,
                 Box::new(file) as Box<dyn Agent + Send + Sync>,
                 Box::new(readme) as Box<dyn Agent + Send + Sync>,
                 Box::new(doc) as Box<dyn Agent + Send + Sync>,
                 Box::new(package) as Box<dyn Agent + Send + Sync>,
                 Box::new(coding) as Box<dyn Agent + Send + Sync>,
                 Box::new(architecture) as Box<dyn Agent + Send + Sync>,
                 Box::new(final_agent) as Box<dyn Agent + Send + Sync>,
                 Box::new(summarization) as Box<dyn Agent + Send + Sync>)
            }
            LlmProvider::OpenRouter => {
                let mut client = openrouter::Client::new(&config.llm.api_key);
                if let Some(base_url) = &config.llm.base_url {
                    client = client.base_url(base_url);
                }

                let basic = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::basic_analysis())
                    .build();
                let file = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::file_analysis())
                    .build();
                let readme = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::readme_analysis())
                    .build();
                let doc = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::documentation_analysis())
                    .build();
                let coding = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::coding_analysis())
                    .build();
                let architecture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::architecture_analysis())
                    .build();
                let package = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::package_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();
                (Box::new(basic) as Box<dyn Agent + Send + Sync>,
                 Box::new(file) as Box<dyn Agent + Send + Sync>,
                 Box::new(readme) as Box<dyn Agent + Send + Sync>,
                 Box::new(doc) as Box<dyn Agent + Send + Sync>,
                 Box::new(package) as Box<dyn Agent + Send + Sync>,
                 Box::new(coding) as Box<dyn Agent + Send + Sync>,
                 Box::new(architecture) as Box<dyn Agent + Send + Sync>,
                 Box::new(final_agent) as Box<dyn Agent + Send + Sync>,
                 Box::new(summarization) as Box<dyn Agent + Send + Sync>)
            }
            LlmProvider::Ollama => {
                let base_url = config
                    .llm
                    .base_url
                    .as_deref()
                    .unwrap_or("http://localhost:11434/v1");
                let client = ollama::Client::new("dummy-key").base_url(base_url);

                let basic = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::basic_analysis())
                    .build();
                let file = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::file_analysis())
                    .build();
                let readme = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::readme_analysis())
                    .build();
                let doc = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::documentation_analysis())
                    .build();
                let coding = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::coding_analysis())
                    .build();
                let architecture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::architecture_analysis())
                    .build();
                let package = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::package_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();
                (Box::new(basic) as Box<dyn Agent + Send + Sync>,
                 Box::new(file) as Box<dyn Agent + Send + Sync>,
                 Box::new(readme) as Box<dyn Agent + Send + Sync>,
                 Box::new(doc) as Box<dyn Agent + Send + Sync>,
                 Box::new(package) as Box<dyn Agent + Send + Sync>,
                 Box::new(coding) as Box<dyn Agent + Send + Sync>,
                 Box::new(architecture) as Box<dyn Agent + Send + Sync>,
                 Box::new(final_agent) as Box<dyn Agent + Send + Sync>,
                 Box::new(summarization) as Box<dyn Agent + Send + Sync>)
            }
        };

        Ok(Self {
            basic_analysis_agent: basic_agent,
            file_analysis_agent: file_agent,
            readme_analysis_agent: readme_agent,
            documentation_analysis_agent: doc_agent,
            package_analysis_agent: package_agent,
            coding_analysis_agent: coding_agent,
            architecture_analysis_agent: architecture_agent,
            final_consolidation_agent: final_agent,
            summarization_agent: summarization_agent,

            provider: config.llm.provider.clone(),
            retry_delay_seconds: config.retry_delay_seconds,
            max_retries: config.max_retries.unwrap_or(3),
        })

    }
   /// Generic retry wrapper for LLM calls with context management
    async fn call_with_retry_context<F, Fut>(&self, agent: &dyn Agent, operation: F) -> Result<String>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<LlmContext>>,
    {
        let mut last_error = None;
        let max_retries = self.max_retries;
        let retry_delay = self.retry_delay_seconds;

        for attempt in 1..=max_retries {
            // Get the context for this attempt
            let mut context = match operation().await {
                Ok(ctx) => ctx,
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        println!("Context preparation failed (attempt {}), retrying in {} seconds...", attempt, retry_delay);
                        sleep(Duration::from_secs(retry_delay)).await;
                        continue;
                    } else {
                        break;
                    }
                }
            };

            // Build the context string with summarization if needed
            let context_str = match context.build_context(&*self.summarization_agent).await {
                Ok(ctx) => ctx,
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        println!("Context building failed (attempt {}), retrying in {} seconds...", attempt, retry_delay);
                        sleep(Duration::from_secs(retry_delay)).await;
                        continue;
                    } else {
                        break;
                    }
                }
            };

            // Make the LLM call
            match agent.prompt(&context_str).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        println!("LLM call failed (attempt {}), retrying in {} seconds...", attempt, retry_delay);
                        sleep(Duration::from_secs(retry_delay)).await;
                    }
                }
            }
        }
        Err(last_error.unwrap())
    }

   /// Generate basic repository analysis with context management
    pub async fn basic_analysis(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.basic_analysis_agent, || async {
            context_builder()
        }).await
    }

    /// Generate README analysis with context management
    pub async fn readme_analysis(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.readme_analysis_agent, || async {
            context_builder()
        }).await
    }

    /// Generate documentation analysis with context management
    pub async fn documentation_analysis(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.documentation_analysis_agent, || async {
            context_builder()
        }).await
    }

    /// Generate package/structure analysis with context management
    pub async fn package_analysis(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.package_analysis_agent, || async {
            context_builder()
        }).await
    }

    /// Generate architecture analysis with context management
    pub async fn architecture_analysis(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.architecture_analysis_agent, || async {
            context_builder()
        }).await
    }

    /// Generate coding analysis with context management
    pub async fn coding_analysis(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.coding_analysis_agent, || async {
            context_builder()
        }).await
    }

    /// Generate file analysis with context management
    pub async fn file_analysis(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.file_analysis_agent, || async {
            context_builder()
        }).await
    }

    /// Generate final consolidation with context management
    pub async fn final_consolidation(&self, context_builder: impl Fn() -> Result<LlmContext>) -> Result<String> {
        self.call_with_retry_context(&*self.final_consolidation_agent, || async {
            context_builder()
        }).await
    }


    /// Get the provider type
    pub fn provider(&self) -> &LlmProvider {
        &self.provider
    }
}

/// System prompts for different analysis phases
pub struct SystemPrompts;

impl SystemPrompts {
    pub fn basic_analysis() -> &'static str {
        r#"You are an expert software architect and documentation specialist. Your task is to analyze a git repository and create comprehensive knowledge documentation.

You will receive information about a repository's basic structure including:
- Repository name and description
- Programming languages used
- Directory structure
- File types and counts

Generate a clear, comprehensive analysis in Markdown format that includes:

1. **Repository Overview**
   - Project name and purpose
   - Primary programming languages
   - Architecture type (library, application, framework, etc.)

2. **Technical Stack**
   - Languages and their usage percentages
   - Key frameworks or libraries identified
   - Build systems detected

3. **Project Structure**
   - High-level directory organization
   - Key directories and their purposes
   - File distribution analysis

Keep the analysis factual, comprehensive, and well-structured. Focus on technical aspects that would help a developer understand the project quickly."#
    }

    pub fn readme_analysis() -> &'static str {
        r#"You are analyzing README files and root-level configuration files to enhance repository knowledge.

You will receive:
- README content (if available)
- Root-level configuration files (package.json, Cargo.toml, etc.)
- License information
- Any other root-level documentation

Update and enhance the existing knowledge with:

1. **Project Description & Purpose**
   - Official project description from README
   - Key features and capabilities
   - Target audience or use cases

2. **Installation & Setup**
   - Dependencies and requirements
   - Installation instructions
   - Development setup process

3. **Usage & Examples**
   - Basic usage examples
   - API overview if applicable
   - Command-line interface details

4. **Configuration**
   - Configuration options
   - Environment variables
   - Build configuration

Integrate this information smoothly with the existing analysis, avoiding duplication while ensuring completeness."#
    }

    pub fn documentation_analysis() -> &'static str {
        r#"You are analyzing documentation files to provide comprehensive project knowledge.

You will receive documentation from:
- docs/ directory
- API documentation
- Architecture documents
- Contributing guidelines
- Changelogs and release notes

Enhance the knowledge base with:

1. **Architecture & Design**
   - System architecture overview
   - Design patterns used
   - Core concepts and abstractions

2. **API Documentation**
   - Public API surface
   - Key modules and their responsibilities
   - Integration patterns

3. **Development Guidelines**
   - Contributing guidelines
   - Code style and conventions
   - Testing approaches

4. **Deployment & Operations**
   - Deployment instructions
   - Configuration management
   - Monitoring and logging

Present the information in a well-organized manner that complements the existing knowledge without redundancy."#
    }

    pub fn coding_analysis() -> &'static str {
        r#"You are creating architecture diagrams and documentation based on your analysis of a software project.

Use Mermaid diagram syntax where possible for clear, readable diagrams. Focus on helping developers understand the system's architecture quickly.

You will receive:
- Detailed directory tree with file information
- Package/module structure
- Import/dependency relationships
- Test organization

Complete the knowledge base with:

1. **Module Architecture**
   - Core modules and their responsibilities
   - Module dependencies and relationships
   - Public vs private interfaces

2. **Code Organization**
   - Package structure rationale
   - Separation of concerns
   - Cross-cutting concerns handling

3. **Testing Strategy**
   - Test organization and structure
   - Testing frameworks used
   - Coverage and quality measures

4. **Build & Deployment**
   - Build process details
   - Packaging and distribution
   - CI/CD pipeline integration

Focus on providing insights that help developers navigate and contribute to the codebase effectively."#
    }

    pub fn architecture_analysis() -> &'static str {
    r#"You are a senior software architect specializing in system design and technical documentation. Your task is to analyze a repository's architecture and create comprehensive visual documentation using Mermaid diagrams.

You will receive:
- Code structure and organization
- Module dependencies and relationships
- Configuration files and deployment setups
- Database schemas and external integrations
- Build and deployment configurations

Generate a detailed architecture analysis in Markdown format that includes:

## 1. System Architecture Diagram
Create a high-level Mermaid diagram showing major components and their relationships:
```mermaid
graph TB
    subgraph "Frontend Layer"
        UI[User Interface]
        API[API Gateway]
    end

    subgraph "Business Layer"
        SVC[Services]
        PROC[Processors]
    end

    subgraph "Data Layer"
        DB[(Database)]
        CACHE[(Cache)]
    end

    UI --> API
    API --> SVC
    SVC --> PROC
    PROC --> DB
    SVC --> CACHE
```

## 2. Component Interaction Flow
Show how main components communicate using sequence diagrams:
```mermaid
sequenceDiagram
    participant Client
    participant API
    participant Service
    participant Database

    Client->>API: Request
    API->>Service: Process
    Service->>Database: Query
    Database->>Service: Data
    Service->>API: Response
    API->>Client: Result
```

## 3. Data Flow Diagram
Illustrate how data moves through the system:
```mermaid
flowchart LR
    INPUT[Input Data] --> VALIDATE[Validation]
    VALIDATE --> PROCESS[Processing]
    PROCESS --> STORE[Storage]
    STORE --> RETRIEVE[Retrieval]
    RETRIEVE --> TRANSFORM[Transformation]
    TRANSFORM --> OUTPUT[Output]
```

## 4. Deployment Architecture
Show runtime components and deployment structure:
```mermaid
graph TB
    subgraph "Production Environment"
        LB[Load Balancer]
        subgraph "Application Tier"
            APP1[App Instance 1]
            APP2[App Instance 2]
        end
        subgraph "Data Tier"
            DB[(Primary DB)]
            REPLICA[(Replica DB)]
        end
        subgraph "Cache Tier"
            REDIS[(Redis Cache)]
        end
    end

    LB --> APP1
    LB --> APP2
    APP1 --> DB
    APP2 --> DB
    APP1 --> REDIS
    APP2 --> REDIS
    DB --> REPLICA
```

## 5. Technology Stack Diagram
Visual representation of technology layers:
```mermaid
graph TB
    subgraph "Technology Stack"
        subgraph "Presentation Layer"
            FRONTEND[Frontend Framework]
            UI_LIB[UI Components]
        end

        subgraph "Application Layer"
            BACKEND[Backend Framework]
            API_LAYER[API Layer]
            BUSINESS[Business Logic]
        end

        subgraph "Data Layer"
            DATABASE[Database Engine]
            ORM[ORM/Data Access]
        end

        subgraph "Infrastructure Layer"
            CONTAINER[Containerization]
            ORCHESTRATION[Orchestration]
            MONITORING[Monitoring]
        end
    end

    FRONTEND --> API_LAYER
    API_LAYER --> BUSINESS
    BUSINESS --> ORM
    ORM --> DATABASE
```

## Guidelines:
- Always use proper Mermaid syntax with appropriate diagram types
- Include clear labels and descriptions for each component
- Show both logical and physical architecture where applicable
- Identify key architectural patterns (MVC, microservices, layered, etc.)
- Highlight critical dependencies and integration points
- Include scalability and performance considerations
- Document security boundaries and data flow restrictions

Focus on creating clear, actionable architectural documentation that helps developers understand the system's design decisions and implementation patterns."#
    }

    pub fn file_analysis() ->  &'static str {
        r#"You are analyzing a specific source code file to understand its role in the project architecture.

Please analyze this file and provide:

1. **Purpose**: What is this file's main responsibility?
2. **Key Functions/Classes**: Main components and their roles
3. **Dependencies**: What other parts of the system does it depend on?
4. **Interfaces**: What APIs, contracts, or interfaces does it define/implement?
5. **Patterns**: What design patterns or architectural patterns are used?
6. **Integration Points**: How does it connect to other system components?

Focus on architectural insights rather than implementation details. Consider how this file fits into the broader system design."#
    }

    pub fn package_analysis() -> &'static str {
        r#"You are analyzing the detailed package and directory structure to complete the repository knowledge.

You will receive:
- Detailed directory tree with file information
- Package/module structure
- Import/dependency relationships
- Test organization

Complete the knowledge base with:

1. **Module Architecture**
   - Core modules and their responsibilities
   - Module dependencies and relationships
   - Public vs private interfaces

2. **Code Organization**
   - Package structure rationale
   - Separation of concerns
   - Cross-cutting concerns handling

3. **Testing Strategy**
   - Test organization and structure
   - Testing frameworks used
   - Coverage and quality measures

4. **Build & Deployment**
   - Build process details
   - Packaging and distribution
   - CI/CD pipeline integration

Focus on providing insights that help developers navigate and contribute to the codebase effectively."#
    }

    pub fn final_consolidation() -> &'static str {
        r#"You are consolidating and finalizing the AI knowledge document for a git repository.

Your task is to:
1. Review the entire knowledge document for consistency and completeness
2. Eliminate any redundancies or contradictions
3. Ensure proper organization and flow
4. Add a comprehensive table of contents
5. Include quick reference sections where appropriate
6. Ensure the document serves as a complete reference for developers

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

The final document should be:
- Well-structured with clear headings
- Comprehensive yet concise
- Easy to navigate and reference
- Technically accurate and up-to-date
- Useful for both new contributors and experienced developers

Format the final document as a professional technical documentation in Markdown."#
    }

    pub fn summarization() -> &'static str {
        "You are a specialized summarization agent. Your task is to create concise, \
         informative summaries of text content while preserving the most important information. \
         When summarizing:

        1. Focus on key facts, main concepts, and critical details
        2. Maintain the original context and meaning
        3. Use clear, concise language
        4. Preserve technical terms and important names/identifiers
        5. Structure the summary logically
        6. Stay within the requested length while maximizing information density

        Always aim to create summaries that allow someone to understand the essential \
        content without reading the full original text."
    }

}
