pub fn default_func_return(name: &str) -> &str {
    if name == "main" {
		"int"
	} else { "void" }
}