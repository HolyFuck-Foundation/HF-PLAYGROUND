fn main() {
    let filename = std::env::args().nth(1).expect("filename given");
    let file = std::fs::read_to_string(&filename).expect("read file");

    let tokens = hf_parser_rust::token::tokenize(&file).expect("successful tokenization");
    let ast = hf_parser_rust::ast::build_ast(tokens).expect("successful ast build");
    print!("{:#?}", ast);

    let ir = hf_codegen::ir::from_ast(ast);

    let target = hf_codegen::target::Target::native();
    let mut compiler = hf_codegen::compiler::HfCompiler::from_target(target);

    #[cfg(target_os = "windows")]
    let mut bytecode = vec![0x49, 0x89, 0xc8];
    #[cfg(target_os = "linux")]
    let mut bytecode = vec![0x49, 0x89, 0xf8];

    bytecode.extend(
        compiler
            .compile_to_bytecode(ir)
            .expect("successful compilation"),
    );

    bytecode.extend(vec![0xC3]);

    unsafe {
        run_code(bytecode);
    }
}

unsafe fn run_code(code: Vec<u8>) {
    use mmap_rs::MmapOptions;
    let mapping = MmapOptions::new(MmapOptions::page_size())
        .unwrap()
        .map()
        .unwrap();
    let mut mmap_mut = mapping.make_mut().unwrap();
    for i in 0..code.len() {
        mmap_mut[i] = code[i];
    }
    let map = mmap_mut.make_exec().unwrap();
    let func: unsafe extern "C" fn(*mut std::ffi::c_void) = std::mem::transmute(map.as_ptr());
    let buffer = vec![0u8; 4096].into_boxed_slice();
    let buffer_ptr = Box::into_raw(buffer) as *mut std::ffi::c_void;
    func(buffer_ptr);
    let _ = unsafe { Box::from_raw(buffer_ptr as *mut [u8; 4096]) }; // Reclaim the memory
    println!("you lived!");
}
