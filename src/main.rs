use std::path::Path;

use hf_codegen::{compiler::CompilerSettings, ir::IrNode};

fn main() {
    let filename = std::env::args().nth(1).expect("filename given");
    let file = std::fs::read_to_string(&filename).expect("read file");

    let tokens = hf_parser_rust::token::tokenize(&file).expect("successful tokenization");
    println!("{} tokens", tokens.len());
    let ast = hf_parser_rust::ast::build_ast(tokens).expect("successful ast build");

    let ir = hf_codegen::ir::from_ast(ast);

    let hf_start = &[IrNode {
        span: hf_codegen::ir::Span {
            location: (0, 0),
            length: 1,
        },
        node: hf_codegen::ir::IrOp::ExternalFunctionCall("hf_start".to_string()),
    }];
    let hf_exit = &[IrNode {
        span: hf_codegen::ir::Span {
            location: (0, 0),
            length: 1,
        },
        node: hf_codegen::ir::IrOp::ExternalFunctionCall("hf_exit".to_string()),
    }];

    let mut real_ir = Vec::from(hf_start);
    real_ir.extend(ir);
    real_ir.extend_from_slice(hf_exit);

    println!("{:#?}", real_ir);

    let target = hf_codegen::target::Target::native();
    let mut compiler = hf_codegen::compiler::HfCompiler::new(target, CompilerSettings::default());

    let obj = compiler.compile_to_object_file(real_ir, &filename).unwrap();
    let raw = obj.write().unwrap();
    let out_filename = Path::new(&filename).with_extension("o");
    std::fs::write(out_filename, raw).unwrap();
}
