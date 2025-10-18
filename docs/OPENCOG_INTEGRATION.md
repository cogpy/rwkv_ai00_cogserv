# OpenCog Integration for AI00 RWKV Server

This document describes the OpenCog Hyperon integration that provides inference engine capabilities to the AI00 RWKV server.

## Overview

The OpenCog integration adds symbolic reasoning and pattern matching capabilities to the RWKV language model server through the Hyperon experimental implementation. This allows for:

- **AtomSpace Operations**: Store and query symbolic knowledge
- **Pattern Matching**: Find relationships and patterns in knowledge
- **MeTTa Evaluation**: Execute MeTTa programming language expressions
- **Inference**: Perform logical reasoning and deduction

## Architecture

### Components

1. **ai00-opencog crate**: Core integration module
   - `atomspace.rs`: AtomSpace management and operations
   - `inference.rs`: MeTTa evaluation and reasoning engine
   - `types.rs`: Type definitions for OpenCog structures

2. **API Endpoints**: REST API for OpenCog operations
   - `/api/opencog/atoms/*`: AtomSpace management
   - `/api/opencog/evaluate`: MeTTa expression evaluation
   - `/api/opencog/infer`: Inference operations
   - `/api/opencog/stats`: AtomSpace statistics

3. **Configuration**: Integration with server config system

### Dependencies

- **hyperon-experimental**: Core OpenCog Hyperon implementation
- **hyperon-space**: AtomSpace functionality
- **hyperon-atom**: Atom type definitions

## Configuration

Add OpenCog configuration to `Config.toml`:

```toml
[opencog] # OpenCog Hyperon inference engine configuration
enabled = true # Enable OpenCog inference engine

[opencog.atomspace] # AtomSpace configuration
initial_size = 1000 # Initial AtomSpace size
persistence = false # Enable persistence
# persistence_path = "assets/atomspace.db" # Path to persistence file

[opencog.inference] # Inference engine configuration
max_steps = 1000 # Maximum inference steps
timeout_ms = 5000 # Timeout for inference operations (milliseconds)
optimize_patterns = true # Enable pattern matching optimization
```

## API Reference

### AtomSpace Operations

#### Add Atom
```
POST /api/opencog/atoms/add
```

Request body:
```json
{
  "id": "uuid-string",
  "type": "Symbol",
  "name": "ConceptName",
  "truth_value": {
    "strength": 0.9,
    "confidence": 0.8
  },
  "attention_value": {
    "sti": 100,
    "lti": 50,
    "vlti": 10
  }
}
```

#### Query Atoms
```
POST /api/opencog/atoms/query
```

Request body:
```json
{
  "pattern": {
    "expression": "ConceptName",
    "context": []
  },
  "bindings": {},
  "max_results": 10
}
```

#### Remove Atom
```
POST /api/opencog/atoms/remove
```

Request body:
```json
{
  "atom_id": "uuid-string"
}
```

#### Clear AtomSpace
```
POST /api/opencog/atoms/clear
```

#### Get Statistics
```
GET /api/opencog/stats
```

Response:
```json
{
  "success": true,
  "stats": {
    "total_atoms": 42,
    "atom_types": {
      "Symbol": 25,
      "Expression": 15,
      "Variable": 2
    },
    "memory_usage": 5376
  }
}
```

### MeTTa Operations

#### Evaluate Expression
```
POST /api/opencog/evaluate
```

Request body:
```json
{
  "expression": "(+ 2 3)",
  "context": []
}
```

#### Perform Inference
```
POST /api/opencog/infer
```

Request body:
```json
{
  "premises": [
    {"expression": "(Inheritance Cat Animal)", "context": []},
    {"expression": "(Inheritance Fluffy Cat)", "context": []}
  ],
  "goal": {"expression": "(Inheritance Fluffy Animal)", "context": []},
  "max_steps": 10,
  "timeout_ms": 5000
}
```

## Usage Examples

### Python Client Example

```python
import requests
import json

# Add a concept
atom = {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "type": "Symbol",
    "name": "Human",
    "truth_value": {"strength": 1.0, "confidence": 0.9}
}

response = requests.post("http://localhost:65530/api/opencog/atoms/add", json=atom)
print(response.json())

# Query for the concept
query = {
    "pattern": {"expression": "Human", "context": []},
    "max_results": 5
}

response = requests.post("http://localhost:65530/api/opencog/atoms/query", json=query)
print(response.json())
```

### MeTTa Expression Examples

```python
# Simple arithmetic
expression = {"expression": "(+ 2 3)", "context": []}
requests.post("http://localhost:65530/api/opencog/evaluate", json=expression)

# Logical inference
inference = {
    "premises": [
        {"expression": "(Inheritance Socrates Human)", "context": []},
        {"expression": "(Inheritance Human Mortal)", "context": []}
    ],
    "goal": {"expression": "(Inheritance Socrates Mortal)", "context": []}
}
requests.post("http://localhost:65530/api/opencog/infer", json=inference)
```

## Integration with RWKV

The OpenCog integration can enhance RWKV generation in several ways:

1. **Knowledge-Guided Generation**: Use AtomSpace knowledge to guide text generation
2. **Reasoning Integration**: Apply logical reasoning to generated content
3. **Pattern-Based Responses**: Use pattern matching to find relevant information
4. **Symbolic Memory**: Maintain symbolic representations of conversations

## Development Status

### Current Features
- ✅ Basic AtomSpace operations (add, remove, query, clear)
- ✅ Configuration integration
- ✅ REST API endpoints
- ✅ Type system for OpenCog structures
- ✅ Statistics and monitoring

### Planned Features
- 🚧 Full MeTTa expression evaluation (threading issues to resolve)
- 🚧 Complete inference engine implementation
- ⭕ RWKV-OpenCog pipeline integration
- ⭕ Persistent AtomSpace storage
- ⭕ Advanced pattern matching algorithms
- ⭕ Multi-modal reasoning support

## Testing

Run the test suite:

```bash
# Start the server with OpenCog enabled
cargo run --bin ai00-server --no-default-features

# Run tests (in another terminal)
python test_opencog.py
```

## Troubleshooting

### Common Issues

1. **Server won't start**: Check that OpenCog is enabled in config
2. **API returns 503**: OpenCog may not be initialized properly
3. **Threading errors**: Some advanced features may require single-threaded operation

### Debug Information

Enable debug logging:
```bash
RUST_LOG=debug cargo run --bin ai00-server
```

## Contributing

When contributing to OpenCog integration:

1. Follow Rust async best practices
2. Maintain thread safety where possible
3. Add comprehensive error handling
4. Update tests and documentation
5. Consider performance implications

## References

- [OpenCog Hyperon](https://hyperon.opencog.org/)
- [Hyperon Experimental Repository](https://github.com/trueagi-io/hyperon-experimental)
- [MeTTa Programming Language](https://wiki.opencog.org/w/MeTTa)
- [AtomSpace Documentation](https://wiki.opencog.org/w/AtomSpace)