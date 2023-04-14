use rigc_core::module::Module;

pub fn generate_c_code(module: &Module) -> String {
	let mut code = String::new();

	for structure in &module.structures {
		let name = structure.name();
		code.push_str(
			format!(
r#"typedef struct {{
    int __placeholder__; // TODO: implement fields
}} {name};

"#).as_str()
		);
	}

	for function in &module.functions {
		let name = function.name();
		let ret = {
			let val = function.return_type();
			if val.is_empty() { "void" } else { val }
		};

		code.push_str(
			format!("{ret} {name}() {{\n\t\n}}\n").as_str()
		);
	}

	code
}