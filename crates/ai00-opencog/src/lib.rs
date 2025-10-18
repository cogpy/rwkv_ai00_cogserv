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