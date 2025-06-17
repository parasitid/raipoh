use crate::config::{Config, LlmProvider};
use crate::error::Result;
use std::time::Duration;
use tokio::time::sleep;

pub trait Agent {
    async fn prompt(&self, prompt: &str) -> crate::Result<String>;
}

use rig::completion::Prompt;
use rig::providers::{anthropic, openai, ollama, openrouter};

/// Unified LLM client that abstracts over different providers
pub struct LlmClient {
    basic_analysis_agent: Box<dyn Agent + Send + Sync>,
    readme_analysis_agent: Box<dyn Agent + Send + Sync>,
    documentation_analysis_agent: Box<dyn Agent + Send + Sync>,
    coding_analysis_agent: Box<dyn Agent + Send + Sync>,
    architecture_analysis_agent: Box<dyn Agent + Send + Sync>,
    package_analysis_agent: Box<dyn Agent + Send + Sync>,
    file_analysis_agent: Box<dyn Agent + Send + Sync>,
    final_consolidation_agent: Box<dyn Agent + Send + Sync>,
    provider: LlmProvider,
    max_retries: u32,
    retry_delay_seconds: u32,
}

impl LlmClient {
    /// Create a new LLM client from configuration
    pub fn new(config: &Config) -> Result<Self> {
        config.validate()?;

        // Create base client based on provider
        let (basic_agent, file_agent, readme_agent, doc_agent, package_agent, coding_agent, architecture_agent, final_agent) = match config.llm.provider {
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
                let archicture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::archicture_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();

                (Box::new(basic), Box::new(readme), Box::new(doc), Box::new(package), Box::new(final_agent))
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
                let archicture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::archicture_analysis())
                    .build();
                let package = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::package_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();

                (Box::new(basic), Box::new(readme), Box::new(doc), Box::new(package), Box::new(final_agent))
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
                let archicture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::archicture_analysis())
                    .build();
                let package = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::package_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();

                (Box::new(basic), Box::new(readme), Box::new(doc), Box::new(package), Box::new(final_agent))
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
                let archicture = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::archicture_analysis())
                    .build();
                let package = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::package_analysis())
                    .build();
                let final_agent = client.agent(&config.llm.model)
                    .preamble(SystemPrompts::final_consolidation())
                    .build();

                (Box::new(basic), Box::new(readme), Box::new(doc), Box::new(package), Box::new(final_agent))
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
            provider: config.llm.provider.clone(),
            retry_delay_seconds: config.retry_delay_seconds,
            max_retries: config.max_retries.unwrap_or(3),
        })

    }

    /// Generic retry wrapper for LLM calls
    async fn call_with_retry<F, Fut>(&self, operation: F) -> Result<String>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<String>>,
    {
        let mut last_error = None;
        let max_retries = self.max_retries;
        let retry_delay = self.retry_delay_seconds;

        for attempt in 1..=max_retries {
            match operation().await {
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

   /// Generate basic repository analysis
    pub async fn basic_analysis(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.basic_analysis_agent.prompt(prompt).await
        }).await
    }

    /// Generate README analysis
    pub async fn readme_analysis(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.readme_analysis_agent.prompt(prompt).await
        }).await
    }

    /// Generate documentation analysis
    pub async fn documentation_analysis(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.documentation_analysis_agent.prompt(prompt).await
        }).await
    }

    /// Generate package/structure analysis
    pub async fn package_analysis(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.package_analysis_agent.prompt(prompt).await
        }).await
    }

    /// Generate architecture analysis with diagrams
    pub async fn architecture_analysis(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.architecture_analysis_agent.prompt(prompt).await
        }).await
    }

    /// Generate coding analysis with diagrams
    pub async fn coding_analysis(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.coding_analysis_agent.prompt(prompt).await
        }).await
    }

    /// Generate file analysis
    pub async fn file_analysis(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.final_consolidation_agent.prompt(prompt).await
        }).await
    }

    /// Generate final consolidation
    pub async fn final_consolidation(&self, prompt: &str) -> Result<String> {
        self.call_with_retry(|| async {
            self.final_consolidation_agent.prompt(prompt).await
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

}
