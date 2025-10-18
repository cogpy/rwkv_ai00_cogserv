//! OpenCog types and structures for AI00 integration

use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use uuid::Uuid;

/// Unique identifier for atoms in the AtomSpace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct AtomId(pub Uuid);

impl AtomId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AtomId {
    fn default() -> Self {
        Self::new()
    }
}

/// Atom type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum AtomType {
    /// Symbol atom (concept)
    Symbol { name: String },
    /// Number atom
    Number { value: f64 },
    /// Expression atom (composite)
    Expression { 
        operator: String, 
        operands: Vec<AtomId> 
    },
    /// Variable atom
    Variable { name: String },
    /// Link atom (relationship)
    Link { 
        relation: String, 
        atoms: Vec<AtomId> 
    },
}

/// Atom representation in the AtomSpace
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Atom {
    /// Unique identifier
    pub id: AtomId,
    /// Atom type and content
    #[serde(flatten)]
    pub atom_type: AtomType,
    /// Truth value (confidence and strength)
    pub truth_value: Option<TruthValue>,
    /// Attention value
    pub attention_value: Option<AttentionValue>,
}

/// Truth value for uncertain reasoning
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TruthValue {
    /// Strength of the truth value (0.0 to 1.0)
    pub strength: f64,
    /// Confidence in the truth value (0.0 to 1.0)
    pub confidence: f64,
}

impl Default for TruthValue {
    fn default() -> Self {
        Self {
            strength: 1.0,
            confidence: 1.0,
        }
    }
}

/// Attention value for atom importance
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttentionValue {
    /// Short-term importance
    pub sti: f64,
    /// Long-term importance  
    pub lti: f64,
    /// Very long-term importance
    pub vlti: f64,
}

impl Default for AttentionValue {
    fn default() -> Self {
        Self {
            sti: 0.0,
            lti: 0.0,
            vlti: 0.0,
        }
    }
}

/// MeTTa expression for pattern matching and evaluation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MettaExpression {
    /// Expression string in MeTTa syntax
    pub expression: String,
    /// Optional context atoms
    pub context: Vec<AtomId>,
}

/// Query pattern for AtomSpace searches
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QueryPattern {
    /// Pattern expression
    pub pattern: MettaExpression,
    /// Variable bindings
    pub bindings: std::collections::HashMap<String, AtomId>,
    /// Maximum number of results
    pub max_results: Option<usize>,
}

/// Query result from pattern matching
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QueryResult {
    /// Matched atoms
    pub matches: Vec<Atom>,
    /// Variable bindings for each match
    pub bindings: Vec<std::collections::HashMap<String, AtomId>>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Inference request for reasoning operations
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InferenceRequest {
    /// Input premises (atoms or expressions)
    pub premises: Vec<MettaExpression>,
    /// Goal to prove or derive
    pub goal: Option<MettaExpression>,
    /// Maximum inference steps
    pub max_steps: Option<usize>,
    /// Timeout in milliseconds
    pub timeout_ms: Option<u64>,
}

/// Inference result from reasoning
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InferenceResult {
    /// Derived conclusions
    pub conclusions: Vec<Atom>,
    /// Inference path/proof
    pub proof_steps: Vec<ProofStep>,
    /// Success status
    pub success: bool,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Single step in an inference proof
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProofStep {
    /// Rule applied
    pub rule: String,
    /// Input atoms
    pub inputs: Vec<AtomId>,
    /// Output atom
    pub output: AtomId,
    /// Step description
    pub description: String,
}

/// AtomSpace statistics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AtomSpaceStats {
    /// Total number of atoms
    pub total_atoms: usize,
    /// Number of different atom types
    pub atom_types: std::collections::HashMap<String, usize>,
    /// Memory usage in bytes
    pub memory_usage: usize,
}