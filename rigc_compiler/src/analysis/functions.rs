use crate::imports::*;

use super::{
	text_span::TextSpan,
	tokens::find_token_of_type
};

pub(crate) struct FoundFunction {
    pub(crate) name: TextSpan,
	pub(crate) return_type: TextSpan,
}

fn search_function_name<'t>(token: &RulePair<'t>) -> TextSpan {
	let name = find_token_of_type(token, Rule::name);

	if name.is_none() {
		return TextSpan::empty();
	}

	let span = name.unwrap().as_span();
	return TextSpan::new(span.start(), span.end());
}

fn search_function_return_type<'t>(token: &RulePair<'t>) -> TextSpan {
	let return_type = find_token_of_type(token, Rule::return_type);

	if return_type.is_none() {
		return TextSpan::empty();
	}	

	let type_ = find_token_of_type(&return_type.unwrap(), Rule::type_);

	if type_.is_none() {
		return TextSpan::empty();
	}

	let span = type_.unwrap().as_span();
	return TextSpan::new(span.start(), span.end());
}

pub(crate) fn analyze_function_definition<'t>(token: RulePair<'t>) -> FoundFunction {
    let name = search_function_name(&token);
	let return_type = search_function_return_type(&token);
    
    FoundFunction {
		name,
		return_type
	}
}