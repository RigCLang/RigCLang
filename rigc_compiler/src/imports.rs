// Extern crates:
extern crate rigc_core;
extern crate pest;

// Imports:
pub use rigc_parser::Rule;

// Types:
pub type Token<'l> = pest::iterators::Pair<'l, Rule>;
