//! Calculator.

use std::collections::HashMap;

use anyhow::*;
use etrace::*;

use super::syntax::{BinOp, Command, Expression};

/// Calculator's context.
#[derive(Debug, Default, Clone)]
pub struct Context {
    anonymous_counter: usize,
    variables: HashMap<String, f64>,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current anonymous variable counter.
    pub fn current_counter(&self) -> usize {
        self.anonymous_counter
    }

    /// Calculates the given expression. (We assume the absence of overflow.)
    pub fn calc_expression(&self, expression: &Expression) -> Result<f64> {
        match expression {
            Expression::Num(value) => Ok(*value),
            Expression::Variable(name) => {
                let val = self
                    .variables
                    .get(name)
                    .ok_or_else(|| anyhow!("undefined variable: {}", name))?;
                Ok(*val)
            }
            Expression::BinOp { op, lhs, rhs } => {
                let left_val = self.calc_expression(lhs)?;
                let right_val = self.calc_expression(rhs)?;
                match op {
                    BinOp::Add => Ok(left_val + right_val),
                    BinOp::Subtract => Ok(left_val - right_val),
                    BinOp::Multiply => Ok(left_val * right_val),
                    BinOp::Divide => {
                        if right_val == 0.0 {
                            bail!("division by zero");
                        }
                        Ok(left_val / right_val)
                    }
                    BinOp::Power => Ok(left_val.powf(right_val)),
                }
            }
        }
    }

    /// Calculates the given command. (We assume the absence of overflow.)
    ///
    /// If there is no variable lhs in the command (i.e. `command.variable = None`), its value
    /// should be stored at `$0`, `$1`, `$2`, ... respectively.
    ///
    /// # Example
    ///
    /// After calculating commad `3 + 5` => Context's variables = `{($0,8)}`
    ///
    /// After calculating commad `v = 3 - 2` => Context's variables = `{($0,8),(v,1))}`
    ///
    /// After calculating commad `3 ^ 2` => Context's variables = `{($0,8),(v,1),($1,9)}`
    pub fn calc_command(&mut self, command: &Command) -> Result<(String, f64)> {
        todo!("fill here")
    }
}
