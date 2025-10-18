//! Inference engine implementation using OpenCog Hyperon

use anyhow::Result;

use hyperon::metta::runner::{Metta, EnvBuilder};
use std::sync::Arc;

use crate::types::{InferenceRequest, InferenceResult, ProofStep, Atom, AtomId, MettaExpression};
use crate::InferenceConfig;

/// Inference engine using Hyperon MeTTa
pub struct InferenceEngine {
    /// Configuration
    config: InferenceConfig,
}

impl InferenceEngine {
    /// Create new inference engine
    pub fn new(config: &InferenceConfig) -> Result<Self> {
        log::info!("Creating inference engine with config: {:?}", config);
        
        Ok(Self {
            config: config.clone(),
        })
    }
    
    /// Create a new MeTTa runner for this operation
    fn create_runner(&self) -> Metta {
        Metta::new(Some(EnvBuilder::test_env()))
    }
    
    /// Execute inference request
    pub async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult> {
        let start_time = std::time::Instant::now();
        
        log::debug!("Starting inference with {} premises", request.premises.len());
        
        let mut conclusions = Vec::new();
        let mut proof_steps = Vec::new();
        let mut success = false;
        
        // Get timeout and max steps from request or config
        let max_steps = request.max_steps.unwrap_or(self.config.max_steps);
        let timeout_ms = request.timeout_ms.unwrap_or(self.config.timeout_ms);
        
        // Set up timeout
        let timeout_future = tokio::time::sleep(tokio::time::Duration::from_millis(timeout_ms));
        tokio::pin!(timeout_future);
        
        // Execute inference steps
        let inference_future = async {
            // Add premises to the MeTTa runner
            let runner = self.create_runner();
            
            for (i, premise) in request.premises.iter().enumerate() {
                match self.add_premise_to_runner(&runner, premise, i).await {
                    Ok(step) => {
                        if let Some(step) = step {
                            proof_steps.push(step);
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to add premise {}: {}", i, e);
                    }
                }
            }
            
            // Execute inference steps
            for step_count in 0..max_steps {
                if let Some(goal) = &request.goal {
                    match self.evaluate_goal(&goal, step_count).await {
                        Ok(Some(result)) => {
                            conclusions.push(result.atom);
                            proof_steps.extend(result.proof_steps);
                            success = true;
                            break;
                        }
                        Ok(None) => {
                            // Continue searching
                        }
                        Err(e) => {
                            log::warn!("Error evaluating goal at step {}: {}", step_count, e);
                            break;
                        }
                    }
                } else {
                    // No specific goal, perform general inference
                    match self.perform_inference_step(step_count).await {
                        Ok(Some(step)) => {
                            if let Some(atom) = step.derived_atom {
                                conclusions.push(atom);
                            }
                            proof_steps.push(step.proof_step);
                        }
                        Ok(None) => {
                            // No more inferences possible
                            success = true;
                            break;
                        }
                        Err(e) => {
                            log::warn!("Error in inference step {}: {}", step_count, e);
                            break;
                        }
                    }
                }
                
                // Check if we should continue
                if conclusions.len() >= 10 {  // Limit number of conclusions
                    success = true;
                    break;
                }
            }
            
            Ok::<(), anyhow::Error>(())
        };
        
        // Execute with timeout
        let _result = tokio::select! {
            _ = &mut timeout_future => {
                log::warn!("Inference timed out after {} ms", timeout_ms);
                success = false;
            }
            result = inference_future => {
                if let Err(e) = result {
                    log::error!("Inference failed: {}", e);
                    success = false;
                }
            }
        };
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        log::debug!("Inference completed: {} conclusions, {} steps, success: {}", 
                   conclusions.len(), proof_steps.len(), success);
        
        Ok(InferenceResult {
            conclusions,
            proof_steps,
            success,
            execution_time_ms: execution_time,
        })
    }
    
    /// Evaluate a MeTTa expression
    pub async fn evaluate_expression(&self, expression: &MettaExpression) -> Result<Vec<hyperon_atom::Atom>> {
        let runner = self.create_runner();
        
        log::debug!("Evaluating expression: {}", expression.expression);
        
        // Use the MeTTa runner to parse and evaluate the expression
        match runner.run(hyperon::metta::text::SExprParser::new(expression.expression.as_str())) {
            Ok(results) => {
                // Flatten the results - MeTTa run returns Vec<Vec<Atom>>
                let flattened: Vec<hyperon_atom::Atom> = results.into_iter().flatten().collect();
                Ok(flattened)
            }
            Err(e) => {
                log::error!("Failed to evaluate expression '{}': {}", expression.expression, e);
                Err(anyhow::anyhow!("Evaluation error: {}", e))
            }
        }
    }
    
    /// Add a premise to the MeTTa runner
    async fn add_premise_to_runner(
        &self,
        _runner: &Metta,
        premise: &MettaExpression,
        index: usize,
    ) -> Result<Option<ProofStep>> {
        // For now, we'll just create a proof step indicating the premise was added
        // In a full implementation, we would parse and add the premise to the space
        log::debug!("Adding premise {}: {}", index, premise.expression);
        
        Ok(Some(ProofStep {
            rule: "add-premise".to_string(),
            inputs: vec![],
            output: AtomId::new(), // Placeholder
            description: format!("Added premise {}: {}", index, premise.expression),
        }))
    }
    
    /// Evaluate a goal expression
    async fn evaluate_goal(&self, goal: &MettaExpression, step: usize) -> Result<Option<GoalResult>> {
        let results = self.evaluate_expression(goal).await?;
        
        if !results.is_empty() {
            // Convert first result to our Atom type
            let hyperon_atom = &results[0];
            let atom = self.hyperon_to_atom(hyperon_atom, AtomId::new())?;
            
            let proof_step = ProofStep {
                rule: "goal-evaluation".to_string(),
                inputs: vec![],
                output: atom.id,
                description: format!("Evaluated goal at step {}: {}", step, goal.expression),
            };
            
            Ok(Some(GoalResult {
                atom,
                proof_steps: vec![proof_step],
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Perform a single inference step
    async fn perform_inference_step(&self, step: usize) -> Result<Option<InferenceStep>> {
        // This is a placeholder for more sophisticated inference
        // In a full implementation, this would apply inference rules
        
        log::debug!("Performing inference step {}", step);
        
        // For now, just return None to indicate no more inferences
        Ok(None)
    }
    
    /// Convert Hyperon Atom to our Atom type
    fn hyperon_to_atom(&self, hyperon_atom: &hyperon_atom::Atom, id: AtomId) -> Result<Atom> {
        use crate::types::{AtomType, TruthValue, AttentionValue};
        
        let atom_type = match hyperon_atom {
            hyperon_atom::Atom::Symbol(sym) => {
                AtomType::Symbol { 
                    name: sym.name().to_string() 
                }
            }
            hyperon_atom::Atom::Variable(var) => {
                AtomType::Variable { 
                    name: var.name().to_string() 
                }
            }
            hyperon_atom::Atom::Expression(expr) => {
                let children = expr.children();
                if !children.is_empty() {
                    if let hyperon_atom::Atom::Symbol(op) = &children[0] {
                        AtomType::Expression {
                            operator: op.name().to_string(),
                            operands: vec![], // TODO: Convert child atoms
                        }
                    } else {
                        AtomType::Expression {
                            operator: "expr".to_string(),
                            operands: vec![],
                        }
                    }
                } else {
                    AtomType::Expression {
                        operator: "empty".to_string(),
                        operands: vec![],
                    }
                }
            }
            hyperon_atom::Atom::Grounded(grounded) => {
                // Try to extract value from grounded atom
                AtomType::Symbol { 
                    name: format!("grounded:{}", grounded.type_()) 
                }
            }
        };
        
        Ok(Atom {
            id,
            atom_type,
            truth_value: Some(TruthValue::default()),
            attention_value: Some(AttentionValue::default()),
        })
    }
}

/// Result of goal evaluation
struct GoalResult {
    atom: Atom,
    proof_steps: Vec<ProofStep>,
}

/// Result of inference step
struct InferenceStep {
    derived_atom: Option<Atom>,
    proof_step: ProofStep,
}

/// Global inference engine instance
static INFERENCE_ENGINE: tokio::sync::OnceCell<Arc<InferenceEngine>> = tokio::sync::OnceCell::const_new();

/// Initialize the global inference engine
pub async fn initialize(config: &InferenceConfig) -> Result<()> {
    let engine = InferenceEngine::new(config)?;
    INFERENCE_ENGINE.set(Arc::new(engine)).map_err(|_| {
        anyhow::anyhow!("Inference engine already initialized")
    })?;
    
    log::info!("Inference engine initialized");
    Ok(())
}

/// Get the global inference engine instance
pub async fn get_inference_engine() -> Result<Arc<InferenceEngine>> {
    INFERENCE_ENGINE.get()
        .ok_or_else(|| anyhow::anyhow!("Inference engine not initialized"))
        .map(Arc::clone)
}

/// Shutdown the inference engine
pub async fn shutdown() -> Result<()> {
    // Currently no cleanup needed for the inference engine
    log::info!("Inference engine shutdown");
    Ok(())
}