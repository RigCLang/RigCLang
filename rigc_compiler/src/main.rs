use std::{env, pin::Pin};

const VERSION: &str = "0.0.1";

extern crate rigc_core;
extern crate pest;

use rigc_core::module::Module;
use rigc_parser::Rule;

fn analyze(mut module: Pin<&mut Module>) {

    let parse_result = rigc_parser::parse(module.content.as_str())
        .expect("Failed to parse input module");

    let mut spans = Vec::new();

    for pair in parse_result {
        if pair.as_rule() == Rule::def_function {
            let name_span = search_function_name(pair);
            spans.push(name_span);
        }
    }

    for name_span in spans {
        module.as_mut().register_function(name_span.0, name_span.1);
    }
}

fn search_function_name<'m>(pair: pest::iterators::Pair<'m, Rule>) -> (usize, usize) {
    for pair in pair.into_inner() {
        if pair.as_rule() == Rule::name {
            let span = pair.as_span();
            return (span.start(), span.end());
        }
    }
    unreachable!("Failed to find function name within a function definition rule!");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("--- RigC Compiler v{} ---", VERSION);
    
    if args.len() < 2 {
        println!("Usage: rigc_compiler <input_file> [output_file: default: out.c]");
        return;
    }

    use std::fs::read_to_string as read_file;

    let mut module = Module::new(
        read_file(args[1].as_str())
            .expect(format!("Failed to read input file: {}", args[1]).as_str())
    );

    analyze(module.as_mut());

    for function in &module.functions {
        println!("Found function: {}", unsafe { &*function.name });
    }

    // for pair in parse {
    //     rigc_parser::display_parse_result(&pair.into());
    // }
}
