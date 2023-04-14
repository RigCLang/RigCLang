// Submodules:
mod imports;
mod analysis;
mod generation;

use generation as gen;


// Imports:
use crate::imports::*;
use std::{env, pin::Pin};
use rigc_core::module::Module;

// Constants
const VERSION: &str = "0.0.1";


fn handle_func_def(token: RulePair, module: Pin<&mut Module>, content: &'static str) {
    use analysis::functions::*;
    let f = analyze_function_definition(token);

    let mut func = module.register_function(
        f.name.start,
        f.name.end
    );

    unsafe {
        func.as_mut().set_return_type(&content[f.return_type.start..f.return_type.end]);
    }
}

fn handle_struct_def(token: RulePair, module: Pin<&mut Module>) {
    use analysis::structures::*;
    let s = analyze_structure_definition(token);

    module.register_structure(
        s.name.start,
        s.name.end
    );
}

fn analyze(content: String) -> Pin<Box<Module>> {
    let mut module = Module::new(content);

    let content = module.as_ref().content_as_static();
    let parse_result = rigc_parser::parse(&content)
        .expect("Failed to parse input module");

    for token in parse_result.clone() {
        if token.as_rule() == Rule::def_function {
            handle_func_def(token, module.as_mut(), content);
        }
        else if token.as_rule() == Rule::def_struct {
            handle_struct_def(token, module.as_mut());
        }
    }

    for pair in parse_result {
        rigc_parser::display_parse_result(&pair);
    }
    
    module
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("--- RigC Compiler v{VERSION} ---");
    
    if args.len() < 2 {
        println!("Usage: rigc_compiler <input_file> [output_file: default: out.c]");
        return;
    }

    use std::fs::read_to_string as read_file;

    let content = read_file(args[1].as_str())
        .expect(format!("Failed to read input file: {}", args[1]).as_str());

    let module = analyze(content);

    for function in &module.functions {
        println!("Found function: {}", function.name());
    }

    let code = gen::c_code::generate(&module);

    use std::fs::write as write_file;
    write_file("out.c", code.as_bytes())
        .expect("Failed to write output file!");
}
