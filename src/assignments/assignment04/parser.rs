#![allow(deprecated)]

//! Parser.

use anyhow::{anyhow, bail, Result};
use etrace::*;
use lazy_static::*;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

use super::syntax::*;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

use inner::*;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pest::prec_climber::{Assoc::*, Operator};
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of
/// plus("add" in our hw) operator is `Left`.
pub fn parse_command(line: &str) -> Result<Command> {
    let mut pairs = SyntaxParser::parse(Rule::command, line)?;
    let first = pairs.next().ok_or_else(|| anyhow!("expected command"))?;

    let (variable, expr_pair) = match first.as_rule() {
        Rule::var => {
            let var_name = first.as_str().to_string();
            let expr_pair = pairs.next().ok_or_else(|| anyhow!("expected expression"))?;
            (Some(var_name), expr_pair)
        }
        Rule::expr => (None, first),
        rule => bail!("unexpected rule in command: {:?}", rule),
    };

    let expression = parse_expression(expr_pair)?;
    Ok(Command {
        variable,
        expression,
    })
}

fn parse_expression(pair: Pair<'_, Rule>) -> Result<Expression> {
    PREC_CLIMBER.climb(
        pair.into_inner(),
        |pair: Pair<'_, Rule>| -> Result<Expression> {
            match pair.as_rule() {
                Rule::num => {
                    let val = pair.as_str().parse::<f64>()?;
                    Ok(Expression::Num(val))
                }
                Rule::var => Ok(Expression::Variable(pair.as_str().to_string())),
                Rule::expr => parse_expression(pair),
                rule => bail!("unexpected rule in primary: {:?}", rule),
            }
        },
        |lhs: Result<Expression>,
         op: Pair<'_, Rule>,
         rhs: Result<Expression>|
         -> Result<Expression> {
            let lhs = lhs?;
            let rhs = rhs?;
            let bin_op = match op.as_rule() {
                Rule::add => BinOp::Add,
                Rule::subtract => BinOp::Subtract,
                Rule::multiply => BinOp::Multiply,
                Rule::divide => BinOp::Divide,
                Rule::power => BinOp::Power,
                rule => bail!("unexpected operator: {:?}", rule),
            };
            Ok(Expression::BinOp {
                op: bin_op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        },
    )
}
