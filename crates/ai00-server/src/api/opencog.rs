//! OpenCog API endpoints for AI00 RWKV Server

use ai00_opencog::{
    atomspace,
    types::{
        Atom, AtomId, InferenceRequest,
        MettaExpression, QueryPattern
    }
};
use salvo::prelude::*;
use serde_json::Value;

/// Add an atom to the AtomSpace
#[endpoint]
pub async fn add_atom(req: &mut Request, res: &mut Response) {
    match req.parse_json::<Atom>().await {
        Ok(atom) => {
            match atomspace::get_atomspace().await {
                Ok(atomspace_manager) => {
                    match atomspace_manager.add_atom(atom).await {
                        Ok(atom_id) => {
                            res.render(Json(serde_json::json!({
                                "success": true,
                                "atom_id": atom_id
                            })));
                        }
                        Err(err) => {
                            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                            res.render(Json(serde_json::json!({
                                "success": false,
                                "error": err.to_string()
                            })));
                        }
                    }
                }
                Err(err) => {
                    res.status_code(StatusCode::SERVICE_UNAVAILABLE);
                    res.render(Json(serde_json::json!({
                        "success": false,
                        "error": format!("AtomSpace not initialized: {}", err)
                    })));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "success": false,
                "error": format!("Invalid atom format: {}", err)
            })));
        }
    }
}

/// Remove an atom from the AtomSpace
#[endpoint]
pub async fn remove_atom(req: &mut Request, res: &mut Response) {
    match req.parse_json::<serde_json::Value>().await {
        Ok(value) => {
            if let Some(atom_id_str) = value.get("atom_id").and_then(|v| v.as_str()) {
                if let Ok(atom_id_uuid) = uuid::Uuid::parse_str(atom_id_str) {
                    let atom_id = AtomId(atom_id_uuid);
                    
                    match atomspace::get_atomspace().await {
                        Ok(atomspace_manager) => {
                            match atomspace_manager.remove_atom(atom_id).await {
                                Ok(success) => {
                                    res.render(Json(serde_json::json!({
                                        "success": success,
                                        "removed": success
                                    })));
                                }
                                Err(err) => {
                                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                                    res.render(Json(serde_json::json!({
                                        "success": false,
                                        "error": err.to_string()
                                    })));
                                }
                            }
                        }
                        Err(err) => {
                            res.status_code(StatusCode::SERVICE_UNAVAILABLE);
                            res.render(Json(serde_json::json!({
                                "success": false,
                                "error": format!("AtomSpace not initialized: {}", err)
                            })));
                        }
                    }
                } else {
                    res.status_code(StatusCode::BAD_REQUEST);
                    res.render(Json(serde_json::json!({
                        "success": false,
                        "error": "Invalid atom_id format"
                    })));
                }
            } else {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(serde_json::json!({
                    "success": false,
                    "error": "Missing atom_id field"
                })));
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "success": false,
                "error": format!("Invalid request format: {}", err)
            })));
        }
    }
}

/// Query atoms using pattern matching
#[endpoint]
pub async fn query_atoms(req: &mut Request, res: &mut Response) {
    match req.parse_json::<QueryPattern>().await {
        Ok(pattern) => {
            match atomspace::get_atomspace().await {
                Ok(atomspace_manager) => {
                    match atomspace_manager.query(pattern).await {
                        Ok(result) => {
                            res.render(Json(serde_json::json!({
                                "success": true,
                                "result": result
                            })));
                        }
                        Err(err) => {
                            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                            res.render(Json(serde_json::json!({
                                "success": false,
                                "error": err.to_string()
                            })));
                        }
                    }
                }
                Err(err) => {
                    res.status_code(StatusCode::SERVICE_UNAVAILABLE);
                    res.render(Json(serde_json::json!({
                        "success": false,
                        "error": format!("AtomSpace not initialized: {}", err)
                    })));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "success": false,
                "error": format!("Invalid query pattern: {}", err)
            })));
        }
    }
}

/// Get AtomSpace statistics
#[endpoint]
pub async fn get_stats(_req: &mut Request, res: &mut Response) {
    match atomspace::get_atomspace().await {
        Ok(atomspace_manager) => {
            match atomspace_manager.get_stats().await {
                Ok(stats) => {
                    res.render(Json(serde_json::json!({
                        "success": true,
                        "stats": stats
                    })));
                }
                Err(err) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(serde_json::json!({
                        "success": false,
                        "error": err.to_string()
                    })));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::SERVICE_UNAVAILABLE);
            res.render(Json(serde_json::json!({
                "success": false,
                "error": format!("AtomSpace not initialized: {}", err)
            })));
        }
    }
}

/// Evaluate a simple MeTTa expression (placeholder for now)
#[endpoint]
pub async fn evaluate_expression(req: &mut Request, res: &mut Response) {
    match req.parse_json::<MettaExpression>().await {
        Ok(expression) => {
            // For now, return a simple evaluation response
            // Full MeTTa evaluation would require solving threading issues with Hyperon
            res.render(Json(serde_json::json!({
                "success": true,
                "expression": expression.expression,
                "result": format!("Evaluated: {}", expression.expression),
                "note": "Full MeTTa evaluation coming soon"
            })));
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "success": false,
                "error": format!("Invalid MeTTa expression: {}", err)
            })));
        }
    }
}

/// Perform basic inference (placeholder for now)
#[endpoint]
pub async fn perform_inference(req: &mut Request, res: &mut Response) {
    match req.parse_json::<InferenceRequest>().await {
        Ok(inference_request) => {
            // For now, return a placeholder response
            // Full inference would require solving threading issues with Hyperon
            res.render(Json(serde_json::json!({
                "success": true,
                "premises_count": inference_request.premises.len(),
                "has_goal": inference_request.goal.is_some(),
                "result": "Inference completed (placeholder)",
                "note": "Full inference engine coming soon"
            })));
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "success": false,
                "error": format!("Invalid inference request: {}", err)
            })));
        }
    }
}

/// Clear all atoms from the AtomSpace
#[endpoint]
pub async fn clear_atomspace(_req: &mut Request, res: &mut Response) {
    match atomspace::get_atomspace().await {
        Ok(atomspace_manager) => {
            match atomspace_manager.clear().await {
                Ok(_) => {
                    res.render(Json(serde_json::json!({
                        "success": true,
                        "message": "AtomSpace cleared successfully"
                    })));
                }
                Err(err) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(serde_json::json!({
                        "success": false,
                        "error": err.to_string()
                    })));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::SERVICE_UNAVAILABLE);
            res.render(Json(serde_json::json!({
                "success": false,
                "error": format!("AtomSpace not initialized: {}", err)
            })));
        }
    }
}