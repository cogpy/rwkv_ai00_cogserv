//! OpenCog Hyperon integration for RWKV AI00 Server
//! 
//! This module provides OpenCog Hyperon-based inference engine capabilities
//! for the AI00 RWKV server, including AtomSpace operations, pattern matching,
//! and MeTTa expression evaluation.

pub mod atomspace;
pub mod inference;
pub mod types;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;

/// OpenCog configuration options
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OpenCogConfig {
    /// Enable OpenCog inference engine
    pub enabled: bool,
    /// AtomSpace backend configuration
    pub atomspace: AtomSpaceConfig,
    /// Inference engine settings
    pub inference: InferenceConfig,
}

impl Default for OpenCogConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            atomspace: AtomSpaceConfig::default(),
            inference: InferenceConfig::default(),
        }
    }
}

/// AtomSpace configuration
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AtomSpaceConfig {
    /// Initial AtomSpace size
    pub initial_size: usize,
    /// Enable persistence
    pub persistence: bool,
    /// Persistence path
    pub persistence_path: Option<String>,
}

impl Default for AtomSpaceConfig {
    fn default() -> Self {
        Self {
            initial_size: 1000,
            persistence: false,
            persistence_path: None,
        }
    }
}

/// Inference engine configuration
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InferenceConfig {
    /// Maximum inference steps
    pub max_steps: usize,
    /// Timeout for inference operations (in milliseconds)
    pub timeout_ms: u64,
    /// Enable pattern matching optimization
    pub optimize_patterns: bool,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            max_steps: 1000,
            timeout_ms: 5000,
            optimize_patterns: true,
        }
    }
}

/// Initialize OpenCog system with given configuration
pub async fn initialize_opencog(config: &OpenCogConfig) -> Result<()> {
    if !config.enabled {
        log::info!("OpenCog inference engine is disabled");
        return Ok(());
    }

    log::info!("Initializing OpenCog inference engine...");
    
    // Initialize AtomSpace
    atomspace::initialize(&config.atomspace).await?;
    
    // Initialize inference engine
    inference::initialize(&config.inference).await?;
    
    log::info!("OpenCog inference engine initialized successfully");
    Ok(())
}

/// Shutdown OpenCog system
pub async fn shutdown_opencog() -> Result<()> {
    log::info!("Shutting down OpenCog inference engine...");
    
    // Shutdown inference engine
    inference::shutdown().await?;
    
    // Shutdown AtomSpace
    atomspace::shutdown().await?;
    
    log::info!("OpenCog inference engine shut down successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[tokio::test]
    async fn test_opencog_initialization() {
        let config = OpenCogConfig {
            enabled: true,
            atomspace: AtomSpaceConfig::default(),
            inference: InferenceConfig::default(),
        };

        // Test initialization
        let result = initialize_opencog(&config).await;
        assert!(result.is_ok());

        // Test AtomSpace operations
        let atomspace = atomspace::get_atomspace().await.unwrap();
        
        // Create test atom
        let atom = Atom {
            id: AtomId::new(),
            atom_type: AtomType::Symbol { name: "TestConcept".to_string() },
            truth_value: Some(TruthValue::default()),
            attention_value: Some(AttentionValue::default()),
        };

        // Add atom
        let atom_id = atomspace.add_atom(atom.clone()).await.unwrap();
        assert_eq!(atom_id, atom.id);

        // Get stats
        let stats = atomspace.get_stats().await.unwrap();
        assert_eq!(stats.total_atoms, 1);

        // Remove atom
        let removed = atomspace.remove_atom(atom_id).await.unwrap();
        assert!(removed);

        // Verify removal
        let stats = atomspace.get_stats().await.unwrap();
        assert_eq!(stats.total_atoms, 0);

        // Test shutdown
        let result = shutdown_opencog().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_disabled_opencog() {
        let config = OpenCogConfig {
            enabled: false,
            atomspace: AtomSpaceConfig::default(),
            inference: InferenceConfig::default(),
        };

        // Should not initialize when disabled
        let result = initialize_opencog(&config).await;
        assert!(result.is_ok());

        // AtomSpace should not be available
        let atomspace_result = atomspace::get_atomspace().await;
        assert!(atomspace_result.is_err());
    }

    #[test]
    fn test_atom_types() {
        // Test AtomId generation
        let id1 = AtomId::new();
        let id2 = AtomId::new();
        assert_ne!(id1, id2);

        // Test AtomType variants
        let symbol = AtomType::Symbol { name: "Test".to_string() };
        let number = AtomType::Number { value: 42.0 };
        let variable = AtomType::Variable { name: "X".to_string() };

        match symbol {
            AtomType::Symbol { name } => assert_eq!(name, "Test"),
            _ => panic!("Expected Symbol type"),
        }

        match number {
            AtomType::Number { value } => assert_eq!(value, 42.0),
            _ => panic!("Expected Number type"),
        }

        match variable {
            AtomType::Variable { name } => assert_eq!(name, "X"),
            _ => panic!("Expected Variable type"),
        }
    }

    #[test]
    fn test_truth_values() {
        let tv = TruthValue::default();
        assert_eq!(tv.strength, 1.0);
        assert_eq!(tv.confidence, 1.0);

        let custom_tv = TruthValue {
            strength: 0.8,
            confidence: 0.9,
        };
        assert_eq!(custom_tv.strength, 0.8);
        assert_eq!(custom_tv.confidence, 0.9);
    }

    #[test]
    fn test_attention_values() {
        let av = AttentionValue::default();
        assert_eq!(av.sti, 0.0);
        assert_eq!(av.lti, 0.0);
        assert_eq!(av.vlti, 0.0);

        let custom_av = AttentionValue {
            sti: 100.0,
            lti: 50.0,
            vlti: 10.0,
        };
        assert_eq!(custom_av.sti, 100.0);
        assert_eq!(custom_av.lti, 50.0);
        assert_eq!(custom_av.vlti, 10.0);
    }

    #[test]
    fn test_metta_expression() {
        let expr = MettaExpression {
            expression: "(+ 2 3)".to_string(),
            context: vec![],
        };
        assert_eq!(expr.expression, "(+ 2 3)");
        assert!(expr.context.is_empty());
    }
}