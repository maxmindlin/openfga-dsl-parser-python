use pyo3::prelude::*;
use pyo3::exceptions::*;

#[pyfunction]
fn dsl_to_json(input: String) -> PyResult<String> {
    let mut parser = openfga_dsl_parser::Parser::new(&input);
    let doc = parser
        .parse_document()
        .map_err(|e| PyValueError::new_err(format!("{e}")))?;

    let json = openfga_dsl_parser::json::JsonTransformer::new(&doc)
        .serialize();
    Ok(json)
}

#[pymodule]
fn openfga_dsl_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dsl_to_json, m)?)?;
    Ok(())
}
