extern crate cpython;

use cpython::{PyErr, Python};

pub fn python_tokenize(sentence: &str) -> Result<Vec<String>, PyErr> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    // the python module packaged in this crate

    let python_code_module = py.import("simple_tokenize").unwrap();
    let result = python_code_module
        .call(py, "simple_tokenize", (sentence,), None)
        .unwrap()
        .extract(py);

    result
}
