extern crate cpython;

use std::env;
use std::path::Path;
use cpython::{PyDict, PyResult, Python};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let python_code_path = Path::new(manifest_dir.as_str()).join("python_code");
    let requirements_path = python_code_path.join("requirements.txt");

    let _res =
        cpython::pip_helpers::install_python_package(python_code_path, Some(requirements_path));

    // set up the nltk tokenizer
    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    locals
        .set_item(py, "nltk", py.import("nltk").unwrap())
        .unwrap();

    let installed: bool = py.eval("nltk.download('punkt')", None, Some(&locals))
        .unwrap()
        .extract(py)
        .unwrap();
    println!("installed the punkt dataset: {}", installed);
}
