use crate::imports::*;

pub(crate)
fn find_token_of_type<'t>(token: &RulePair<'t>, rule: Rule) -> Option<RulePair<'t>>
{
	for pair in token.clone().into_inner() {
		if pair.as_rule() == rule {
			return Some(pair);
		}
	}

	None
}