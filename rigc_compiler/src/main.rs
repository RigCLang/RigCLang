// Submodules:
mod imports;
mod analysis;
mod generation;


// Imports:
use crate::imports::*;
use std::{env, pin::Pin};
use rigc_core::module::Module;

// Constants
const VERSION: &str = "0.0.1";

fn analyze(content: String) -> Pin<Box<Module>> {
    use analysis::functions::*;
    use analysis::structures::*;

    let mut module = Module::new(content);

    let content = module.as_ref().content_as_static();
    let parse_result = rigc_parser::parse(&content)
        .expect("Failed to parse input module");

    for token in parse_result {
        if token.as_rule() == Rule::def_function {
            let f = analyze_function_definition(token);

            let mut func = module.as_mut().register_function(
                f.name.start,
                f.name.end
            );

            unsafe {
                func.as_mut().set_return_type(&content[f.return_type.start..f.return_type.end]);
            }
        }
        else if token.as_rule() == Rule::def_struct {
            let s = analyze_structure_definition(token);

            module.as_mut().register_structure(
                s.name.start,
                s.name.end
            );
        }
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

    let code = generation::generate_c_code(&module);

    // write it to out.c file
    use std::fs::write as write_file;
    write_file("out.c", code.as_bytes())
        .expect("Failed to write output file!");


    // for pair in parse {
    //     rigc_parser::display_parse_result(&pair.into());
    // }
}
