// Import PEST library
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use pest::{Parser, pratt_parser::PrattParser, iterators::Pairs, iterators::Pair};
///////////////////////////

#[derive(Parser)]
#[grammar = "rigc.pest"]
struct RigCParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(
                // =
                Op::infix(op_assign, Left) |
                // += -=
                Op::infix(op_add_assign, Left) | Op::infix(op_sub_assign, Left) |
                // *= /=
                Op::infix(op_mul_assign, Left) | Op::infix(op_div_assign, Left) |
                // %=
                Op::infix(op_mod_assign, Left)
            )
            .op(Op::infix(op_or, Left))
            .op(Op::infix(op_and, Left))
            .op(Op::infix(op_rel_eq, Left) | Op::infix(op_rel_neq, Left))
            .op(
                Op::infix(op_rel_lt, Left) | Op::infix(op_rel_gt, Left) |
                Op::infix(op_rel_lte, Left) | Op::infix(op_rel_gte, Left)
            )
            .op(Op::infix(op_add, Left) | Op::infix(op_sub, Left)) // + -
            .op(Op::infix(op_mul, Left) | Op::infix(op_div, Left) | Op::infix(op_mod, Left)) // * / %
            .op(Op::prefix(op_unary_minus))
            .op(Op::postfix(op_func_call) | Op::postfix(op_subscript))
    };
}

pub type ParseResult<'a> = Result<
        Pairs<'a, Rule>,
        pest::error::Error<Rule>
    >;

#[derive(Debug)]
pub enum Expression<'a> {
    PrefixOperator {
        op: Pair<'a, Rule>,
        right: Box<Expression<'a>>,
    },
    InfixOperator {
        left: Box<Expression<'a>>,
        op: Pair<'a, Rule>,
        right: Box<Expression<'a>>,
    },
    PostfixOperator {
        left: Box<Expression<'a>>,
        op: Pair<'a, Rule>,
    },
    Atom(Pair<'a, Rule>),
}

impl<'a> std::fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::PrefixOperator { op, right } => {
                write!(f, "({} {})", op.as_str(), right)
            }
            Expression::InfixOperator { left, op, right } => {
                write!(f, "({} {} {})", left, op.as_str(), right)
            }  
            Expression::PostfixOperator { left, op } => {
                write!(f, "({} {})", left, op.as_str())
            }
            Expression::Atom(atom) => {
                write!(f, "{}", atom.as_str())
            }
        }
    }
}


pub type ModuleParseResult<'a> = Option<Pair<'a, Rule>>;

pub fn parse<'m>(module: &'m str) -> ParseResult<'m> {
    RigCParser::parse(Rule::module, module)
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::li_integer => Expression::Atom(primary),
            Rule::li_float => Expression::Atom(primary),
            Rule::scoped_name => Expression::Atom(primary),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expression::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            Expression::InfixOperator {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs|
            Expression::PrefixOperator {
                op,
                right: Box::new(rhs)
            }
        )
        .parse(pairs)
}

pub fn display_parse_result(pair: &Pair<'_, Rule>) {
    display_parse_result_indented(pair, 0);
}


fn display_parse_result_indented(pair: &Pair<'_, Rule>, indent: usize) {

    print!("{}", " ".repeat(indent * 2));

    if pair.as_rule() == Rule::expr {
        println!("Expression:");
        println!("{}", parse_expr(pair.clone().into_inner()));
        return;
    }
    
    println!("- {:?} ({:?})", pair.as_rule(), pair.as_str());

    // display sub rules
    for sub_pair in pair.clone().into_inner() {
        display_parse_result_indented(&sub_pair, indent + 1);
    }
}