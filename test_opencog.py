#!/usr/bin/env python3
"""
Test script for OpenCog integration with AI00 RWKV Server

This script demonstrates the OpenCog API endpoints and functionality.
"""

import json
import requests
import uuid

# Server configuration
BASE_URL = "http://localhost:65530"
OPENCOG_API = f"{BASE_URL}/api/opencog"

def test_atomspace_stats():
    """Test getting AtomSpace statistics"""
    print("Testing AtomSpace statistics...")
    response = requests.get(f"{OPENCOG_API}/stats")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    print()

def test_add_atom():
    """Test adding an atom to AtomSpace"""
    print("Testing atom addition...")
    
    # Create a test atom
    atom = {
        "id": str(uuid.uuid4()),
        "type": "Symbol",
        "name": "TestConcept",
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
    
    response = requests.post(f"{OPENCOG_API}/atoms/add", json=atom)
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    
    if response.status_code == 200:
        result = response.json()
        if result.get("success"):
            return result.get("atom_id")
    return None

def test_query_atoms():
    """Test querying atoms with patterns"""
    print("Testing atom query...")
    
    query = {
        "pattern": {
            "expression": "TestConcept",
            "context": []
        },
        "bindings": {},
        "max_results": 10
    }
    
    response = requests.post(f"{OPENCOG_API}/atoms/query", json=query)
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    print()

def test_evaluate_expression():
    """Test MeTTa expression evaluation"""
    print("Testing MeTTa expression evaluation...")
    
    expression = {
        "expression": "(+ 2 3)",
        "context": []
    }
    
    response = requests.post(f"{OPENCOG_API}/evaluate", json=expression)
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    print()

def test_inference():
    """Test basic inference"""
    print("Testing inference...")
    
    inference_request = {
        "premises": [
            {"expression": "(Inheritance Cat Animal)", "context": []},
            {"expression": "(Inheritance Fluffy Cat)", "context": []}
        ],
        "goal": {"expression": "(Inheritance Fluffy Animal)", "context": []},
        "max_steps": 10,
        "timeout_ms": 5000
    }
    
    response = requests.post(f"{OPENCOG_API}/infer", json=inference_request)
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    print()

def test_remove_atom(atom_id):
    """Test removing an atom from AtomSpace"""
    if not atom_id:
        print("Skipping atom removal (no atom to remove)")
        return
        
    print("Testing atom removal...")
    
    removal_request = {
        "atom_id": atom_id
    }
    
    response = requests.post(f"{OPENCOG_API}/atoms/remove", json=removal_request)
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    print()

def test_clear_atomspace():
    """Test clearing the AtomSpace"""
    print("Testing AtomSpace clear...")
    
    response = requests.post(f"{OPENCOG_API}/atoms/clear")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    print()

def main():
    """Run all tests"""
    print("OpenCog Integration Test for AI00 RWKV Server")
    print("=" * 50)
    print()
    
    try:
        # Test basic functionality
        test_atomspace_stats()
        
        # Test atom operations
        atom_id = test_add_atom()
        test_atomspace_stats()  # Check stats after adding
        test_query_atoms()
        
        # Test inference and evaluation
        test_evaluate_expression()
        test_inference()
        
        # Cleanup
        test_remove_atom(atom_id)
        test_clear_atomspace()
        test_atomspace_stats()  # Check stats after clearing
        
        print("All tests completed!")
        
    except requests.exceptions.ConnectionError:
        print(f"Error: Could not connect to server at {BASE_URL}")
        print("Make sure the AI00 server is running with OpenCog enabled.")
    except Exception as e:
        print(f"Error during testing: {e}")

if __name__ == "__main__":
    main()