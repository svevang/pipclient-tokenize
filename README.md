# pipclient-tokenize
A simple rust crate that calls into python's nltk and tokenizes a string.

Based on my fork of [rust-cpython](https://github.com/svevang/rust-cpython/tree/feature/python-as-crate), pipclient-tokenize allows a simple invocation of NLTK
s tokenize function to be packaged as a crate.

## Quickstart

Build with rust as usual: `cargo build` and then run the main to see some hard coded input that's tokenized and then printed:

```
±  |master ✗| → ./target/debug/pipclient-tokenize
Result: Ok(["Hello", ",", "how", "are", "you", "?", "I", "am", "fine", "thank", "you", "."])
```
