use std::path::Path;

use hf_codegen::compiler::CompilerSettings;

fn main() {
    let filename = std::env::args().nth(1).expect("filename given");
    let file = std::fs::read_to_string(&filename).expect("read file");

    let tokens = hf_parser_rust::token::tokenize(&file).expect("successful tokenization");
    let ast = hf_parser_rust::ast::build_ast(tokens).expect("successful ast build");
    println!("{:#?}", ast);

    let ir = hf_codegen::ir::from_ast(ast);
    println!("{:#?}", ir);

    let target = hf_codegen::target::Target::native();
    let mut compiler = hf_codegen::compiler::HfCompiler::new(target, CompilerSettings::default());

    let obj = compiler
        .compile_to_object_file(ir, &filename)
        .unwrap();
    let raw = obj.write().unwrap();
    let out_filename = Path::new(&filename).with_extension("o");
    std::fs::write(out_filename, raw).unwrap();
}
