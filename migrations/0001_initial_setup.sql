-- Create analysis_steps table
CREATE TABLE IF NOT EXISTS analysis_steps (
    id TEXT PRIMARY KEY,
    step_type TEXT NOT NULL,
    status TEXT NOT NULL,
    input_data TEXT NOT NULL,
    output_data TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL,
    completed_at TEXT
);

-- Create knowledge_entries table
CREATE TABLE IF NOT EXISTS knowledge_entries (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    subcategory TEXT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    relevance_score REAL NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_analysis_steps_status ON analysis_steps(status);
CREATE INDEX IF NOT EXISTS idx_analysis_steps_created_at ON analysis_steps(created_at);
CREATE INDEX IF NOT EXISTS idx_knowledge_entries_category ON knowledge_entries(category);
CREATE INDEX IF NOT EXISTS idx_knowledge_entries_relevance ON knowledge_entries(relevance_score DESC);
