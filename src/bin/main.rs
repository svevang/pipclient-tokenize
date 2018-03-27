extern crate pipclient_tokenize;

fn main() {
    let tokenized_result =
        pipclient_tokenize::python_tokenize("Hello, how are you? I am fine thank you.");
    println!("Result: {:?}", tokenized_result);
}
