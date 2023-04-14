use crate::imports::*;

use super::{
	text_span::TextSpan,
	tokens::find_token_of_type
};

pub(crate) struct FoundStructure {
    pub(crate) name: TextSpan,
}

fn search_structure_name<'t>(token: &RulePair<'t>) -> TextSpan {
	let name = find_token_of_type(token, Rule::name);

	if name.is_none() {
		return TextSpan::empty();
	}

	let span = name.unwrap().as_span();
	return TextSpan::new(span.start(), span.end());
}

pub(crate) fn analyze_structure_definition<'t>(token: RulePair<'t>) -> FoundStructure {
    let name = search_structure_name(&token);
    
    FoundStructure {
		name,
	}
}