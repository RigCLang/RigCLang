extern crate pest;

// Imports:
pub use rigc_parser::Rule;

// Types:
pub type RulePair<'l> = pest::iterators::Pair<'l, Rule>;

