# OpenFGA DSL Parser (Python)

The OpenFGA DSL Parser provides utilities for parsing DSL file string inputs into an AST representation, as well as transforming them into a JSON representation.

This library is meant to be a utility for services using [OpenFGA](https://openfga.dev/) for their authorization solution, but need a way to translate from their DSL to the JSON format the HTTP API expects.

## Usage

```python
from openfga_dsl_parser import dsl_to_json
input = """type group
  relations
    define member as self
type resource
  relations
    define writer as self
    define reader as self but not writer"""

json = dsl_to_json(input)
print(json)
```

## Development

Setup Python environment & install Maturin:

1. `python -m venv .nenv`
2. `source .venv/bin/activate`
3. `pip install maturin`

Install & build package using Maturin for local testing:

`maturin develop`
