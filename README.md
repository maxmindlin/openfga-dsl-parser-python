# OpenFGA DSL Parser (Python)

The OpenFGA DSL Parser provides utilities for parsing DSL file string inputs into an AST representation, as well as transforming them into a JSON representation.

This library is meant to be a utility for services using OpenFGA for their authorization solution, but need a way to translate from their DSL to the JSON format the HTTP API expects.

## Development

Setup Python environment & install Maturin:

1. `python -m venv .nenv`
2. `source .venv/bin/activate`
3. `pip install maturin`

Install & build package using Maturin for local testing:

`maturin develop`
