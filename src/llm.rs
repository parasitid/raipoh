use crate::config::{Config, LlmProvider};
use crate::error::Result;
use rig::completion::{CompletionModel, Prompt};
use rig::providers::{anthropic, openai};

/// Unified LLM client that abstracts over different providers
pub struct LlmClient {
    model: Box<dyn CompletionModel + Send + Sync>,
    provider: LlmProvider,
}

impl LlmClient {
    /// Create a new LLM client from configuration
    pub fn new(config: &Config) -> Result<Self> {
        config.validate()?;

        let model: Box<dyn CompletionModel + Send + Sync> = match config.llm.provider {
            LlmProvider::OpenAI => {
                let client = openai::Client::new(&config.llm.api_key);
                let model = client.model(&config.llm.model);
                Box::new(model)
            }
            LlmProvider::Anthropic => {
                let client = anthropic::Client::new(&config.llm.api_key);
                let model = client.model(&config.llm.model);
                Box::new(model)
            }
            LlmProvider::OpenRouter => {
                // OpenRouter typically uses OpenAI-compatible API
                let mut client = openai::Client::new(&config.llm.api_key);
                if let Some(base_url) = &config.llm.base_url {
                    client = client.base_url(base_url);
                }
                let model = client.model(&config.llm.model);
                Box::new(model)
            }
            LlmProvider::Ollama => {
                // Ollama uses OpenAI-compatible API on localhost
                let base_url = config
                    .llm
                    .base_url
                    .as_deref()
                    .unwrap_or("http://localhost:11434/v1");
                let client = openai::Client::new("dummy-key").base_url(base_url);
                let model = client.model(&config.llm.model);
                Box::new(model)
            }
        };

        Ok(Self {
            model,
            provider: config.llm.provider.clone(),
        })
    }

    /// Generate a completion for the given prompt
    pub async fn complete(&self, prompt: &str) -> Result<String> {
        let prompt = dyn Prompt::Text(prompt.to_string());
        let response = self.model.completion(prompt).await?;
        Ok(response.content)
    }

    /// Generate a completion with system prompt
    pub async fn complete_with_system(&self, system: &str, user: &str) -> Result<String> {
        let messages = vec![
            ("system".to_string(), system.to_string()),
            ("user".to_string(), user.to_string()),
        ];
        let prompt = dyn Prompt::Chat(messages);
        let response = self.model.completion(prompt).await?;
        Ok(response.content)
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

The final document should be:
- Well-structured with clear headings
- Comprehensive yet concise
- Easy to navigate and reference
- Technically accurate and up-to-date
- Useful for both new contributors and experienced developers

Format the final document as a professional technical documentation in Markdown."#
    }
}
