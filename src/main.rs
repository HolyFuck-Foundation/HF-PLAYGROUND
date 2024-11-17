fn main() {
    let filename = std::env::args().nth(1).expect("filename given");
    let file = std::fs::read_to_string(&filename).expect("read file");

    let tokens = hf_parser_rust::tokenize(&file).expect("successful tokenization");
    let ast = hf_parser_rust::build_ast(tokens).expect("successful AST construction");
    print!("{:#?}", ast);
}
