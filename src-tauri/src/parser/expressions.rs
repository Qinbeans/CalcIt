use super::{constants::{Constant}, tokens::Token};
use serde::{Deserialize, Serialize};

pub trait Expression {
    fn evaluate(&self) -> Constant;
}

pub trait Unary {
    fn new(expression: Box<EnumExpression>) -> Self;
}

pub trait Binary {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConstExpression {
    pub constant: Constant,
}

impl Expression for ConstExpression {
    fn evaluate(&self) -> Constant {
        self.constant
    }
}

impl ConstExpression {
    pub fn new(token: Token) -> Self {
        Self { constant: Constant::from_tok(token) }
    }
}
/**
 * Subtraction expression (left - right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct SubtractExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for SubtractExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Float(right)) => Constant::Float(left as f64 - right),
            (Constant::Binary(left), Constant::Float(right)) => Constant::Float(left as f64 - right),
            (Constant::Hexadecimal(left), Constant::Float(right)) => Constant::Float(left as f64 - right),
            (Constant::Octal(left), Constant::Float(right)) => Constant::Float(left as f64 - right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Float(left - right as f64),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Float(left - right as f64),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Float(left - right as f64),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Float(left - right as f64),
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left - right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Integer(left - right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Integer(left - right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Integer(left - right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Integer(left - right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left - right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left - right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left - right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Integer(left - right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left - right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left - right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left - right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Integer(left - right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left - right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left - right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left - right),
            _ => panic!("Invalid subtraction operation"),
        }
    }
}

impl Binary for SubtractExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> SubtractExpression {
        SubtractExpression { left, right }
    }
}
/**
 * Addition expression (left + right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct AddExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for AddExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Float(right)) => Constant::Float(left as f64 + right),
            (Constant::Binary(left), Constant::Float(right)) => Constant::Float(left as f64 + right),
            (Constant::Hexadecimal(left), Constant::Float(right)) => Constant::Float(left as f64 + right),
            (Constant::Octal(left), Constant::Float(right)) => Constant::Float(left as f64 + right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Float(left + right as f64),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Float(left + right as f64),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Float(left + right as f64),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Float(left + right as f64),
            (Constant::Float(left), Constant::Float(right)) => Constant::Float(left + right),
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left + right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Integer(left + right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Integer(left + right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Integer(left + right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Integer(left + right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left + right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left + right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left + right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Integer(left + right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left + right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left + right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left + right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Integer(left + right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left + right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left + right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left + right),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Binary for AddExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> AddExpression {
        AddExpression { left, right }
    }
}
/**
 * Multiplication expression (left * right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct MultiplyExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for MultiplyExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Float(right)) => Constant::Float(left as f64 * right),
            (Constant::Binary(left), Constant::Float(right)) => Constant::Float(left as f64 * right),
            (Constant::Hexadecimal(left), Constant::Float(right)) => Constant::Float(left as f64 * right),
            (Constant::Octal(left), Constant::Float(right)) => Constant::Float(left as f64 * right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Float(left * right as f64),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Float(left * right as f64),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Float(left * right as f64),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Float(left * right as f64),
            (Constant::Float(left), Constant::Float(right)) => Constant::Float(left * right),
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left * right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Integer(left * right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Integer(left * right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Integer(left * right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Integer(left * right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left * right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left * right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left * right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Integer(left * right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left * right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left * right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left * right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Integer(left * right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left * right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left * right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left * right),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Binary for MultiplyExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> MultiplyExpression {
        MultiplyExpression { left, right }
    }
}
/**
 * Division expression (left / right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct DivideExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for DivideExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Float(right)) => Constant::Float(left as f64 / right),
            (Constant::Binary(left), Constant::Float(right)) => Constant::Float(left as f64 / right),
            (Constant::Hexadecimal(left), Constant::Float(right)) => Constant::Float(left as f64 / right),
            (Constant::Octal(left), Constant::Float(right)) => Constant::Float(left as f64 / right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Float(left / right as f64),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Float(left / right as f64),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Float(left / right as f64),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Float(left / right as f64),
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left / right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Integer(left / right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Integer(left / right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Integer(left / right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Integer(left / right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left / right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left / right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left / right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Integer(left / right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left / right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left / right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left / right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Integer(left / right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left / right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left / right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left / right),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Binary for DivideExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> DivideExpression {
        DivideExpression { left, right }
    }
}
/**
 * Modulo expression (left % right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct ModuloExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for ModuloExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Float(right)) => Constant::Float(left as f64 % right),
            (Constant::Binary(left), Constant::Float(right)) => Constant::Float(left as f64 % right),
            (Constant::Hexadecimal(left), Constant::Float(right)) => Constant::Float(left as f64 % right),
            (Constant::Octal(left), Constant::Float(right)) => Constant::Float(left as f64 % right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Float(left % right as f64),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Float(left % right as f64),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Float(left % right as f64),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Float(left % right as f64),
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left % right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Integer(left % right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Integer(left % right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Integer(left % right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Integer(left % right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left % right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left % right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left % right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Integer(left % right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left % right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left % right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left % right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Integer(left % right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left % right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left % right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left % right),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Binary for ModuloExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> ModuloExpression {
        ModuloExpression { left, right }
    }
}

/**
 * Exponentiation expression (left ** right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct ExponentExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for ExponentExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Float(right)) => Constant::Float((left as f64).powf(right)),
            (Constant::Binary(left), Constant::Float(right)) => Constant::Float((left as f64).powf(right)),
            (Constant::Hexadecimal(left), Constant::Float(right)) => Constant::Float((left as f64).powf(right)),
            (Constant::Octal(left), Constant::Float(right)) => Constant::Float((left as f64).powf(right)),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Float(left.powf(right as f64)),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Float(left.powf(right as f64)),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Float(left.powf(right as f64)),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Float(left.powf(right as f64)),
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left.pow(right as u32)),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Integer(left.pow(right as u32)),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Integer(left.pow(right as u32)),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Integer(left.pow(right as u32)),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Integer(left.pow(right as u32)),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left.pow(right as u32)),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left.pow(right as u32)),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left.pow(right as u32)),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Integer(left.pow(right as u32)),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left.pow(right as u32)),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left.pow(right as u32)),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left.pow(right as u32)),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Integer(left.pow(right as u32)),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left.pow(right as u32)),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left.pow(right as u32)),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left.pow(right as u32)),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Binary for ExponentExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self {
        Self { left, right }
    }
}
/**
 * Root expression (left // right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct RootExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for RootExpression {
    //the right must be an integer of some sort
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Binary((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Hexadecimal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Octal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Float(left.powf(1.0 / right as f64)),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Integer((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Hexadecimal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Octal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Float(left.powf(1.0 / right as f64)),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Integer((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Octal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Float(left.powf(1.0 / right as f64)),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Integer((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal((left as f64).powf(1.0 / right as f64) as i64),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Float(left.powf(1.0 / right as f64)),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Binary for RootExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self {
        Self { left, right }
    }
}
/**
 * Bitwise XOR expression (left ^ right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct XorExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for XorExpression {
    //the right must be an integer of some sort
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left ^ right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Binary(left ^ right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Hexadecimal(left ^ right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Octal(left ^ right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Integer((left as i64) ^ right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Binary(left ^ right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left ^ right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left ^ right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left ^ right),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Integer((left as i64) ^ right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left ^ right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left ^ right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left ^ right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left ^ right),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Integer((left as i64) ^ right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Octal(left ^ right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left ^ right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left ^ right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left ^ right),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Integer((left as i64) ^ right),
            _ => panic!("Invalid xor operation"),
        }
    }
}

impl Binary for XorExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self {
        Self { left, right }
    }
}
/**
 * Bitwise AND expression (left & right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct AndExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for AndExpression {
    //the right must be an integer of some sort
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left & right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Binary(left & right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Hexadecimal(left & right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Octal(left & right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Binary(left & right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left & right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left & right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left & right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left & right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left & right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left & right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left & right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Octal(left & right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left & right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left & right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left & right),
            _ => panic!("Invalid and operation"),
        }
    }
}

impl Binary for AndExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self {
        Self { left, right }
    }
}
/**
 * Bitwise OR expression (left | right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct OrExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for OrExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left | right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Binary(left | right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Hexadecimal(left | right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Octal(left | right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Integer((left as i64) | right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Binary(left | right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left | right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left | right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left | right),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Integer((left as i64) | right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left | right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left | right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left & right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left | right),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Integer((left as i64) | right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Octal(left | right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left | right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left | right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left | right),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Integer((left as i64) | right),
            _ => panic!("Invalid or operation"),
        }
    }
}

impl Binary for OrExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self {
        Self { left, right }
    }
}
/**
 * Bitwise Left Shift expression (left << right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct LeftShiftExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for LeftShiftExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left << right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Binary(left << right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Hexadecimal(left << right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Octal(left << right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Integer((left as i64) << right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Binary(left << right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left << right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left << right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left << right),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Integer((left as i64) << right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left << right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left << right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left << right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left << right),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Integer((left as i64) << right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Octal(left << right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left << right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left << right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left << right),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Integer((left as i64) << right),
            _ => panic!("Invalid shift operation"),
        }
    }
}

impl Binary for LeftShiftExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self {
        Self { left, right }
    }
}
/**
 * Bitwise Right Shift expression (left >> right) implementation
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct RightShiftExpression {
    pub left: Box<EnumExpression>,
    pub right: Box<EnumExpression>,
}

impl Expression for RightShiftExpression {
    fn evaluate(&self) -> Constant {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        //match the left and right values, left value holds precedence
        match (left, right) {
            (Constant::Integer(left), Constant::Integer(right)) => Constant::Integer(left >> right),
            (Constant::Binary(left), Constant::Integer(right)) => Constant::Binary(left >> right),
            (Constant::Hexadecimal(left), Constant::Integer(right)) => Constant::Hexadecimal(left >> right),
            (Constant::Octal(left), Constant::Integer(right)) => Constant::Octal(left >> right),
            (Constant::Float(left), Constant::Integer(right)) => Constant::Integer((left as i64) >> right),
            (Constant::Integer(left), Constant::Binary(right)) => Constant::Binary(left >> right),
            (Constant::Binary(left), Constant::Binary(right)) => Constant::Binary(left >> right),
            (Constant::Hexadecimal(left), Constant::Binary(right)) => Constant::Binary(left >> right),
            (Constant::Octal(left), Constant::Binary(right)) => Constant::Binary(left >> right),
            (Constant::Float(left), Constant::Binary(right)) => Constant::Integer((left as i64) >> right),
            (Constant::Integer(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left >> right),
            (Constant::Binary(left), Constant::Hexadecimal(right)) => Constant::Binary(left >> right),
            (Constant::Hexadecimal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left >> right),
            (Constant::Octal(left), Constant::Hexadecimal(right)) => Constant::Hexadecimal(left >> right),
            (Constant::Float(left), Constant::Hexadecimal(right)) => Constant::Integer((left as i64) >> right),
            (Constant::Integer(left), Constant::Octal(right)) => Constant::Octal(left >> right),
            (Constant::Binary(left), Constant::Octal(right)) => Constant::Binary(left >> right),
            (Constant::Hexadecimal(left), Constant::Octal(right)) => Constant::Hexadecimal(left >> right),
            (Constant::Octal(left), Constant::Octal(right)) => Constant::Octal(left >> right),
            (Constant::Float(left), Constant::Octal(right)) => Constant::Integer((left as i64) >> right),
            _ => panic!("Invalid shift operation"),
        }
    }
}

impl Binary for RightShiftExpression {
    fn new(left: Box<EnumExpression>, right: Box<EnumExpression>) -> Self {
        Self { left, right }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum BinaryExpression {
    Addition(AddExpression),
    Subtraction(SubtractExpression),
    Multiplication(MultiplyExpression),
    Division(DivideExpression),
    Modulus(ModuloExpression),
    Exponent(ExponentExpression),
    Root(RootExpression),
    BitwiseAnd(AndExpression),
    BitwiseOr(OrExpression),
    BitwiseXor(XorExpression),
    LeftShift(LeftShiftExpression),
    RightShift(RightShiftExpression),
}

impl BinaryExpression {
    pub fn evaluate(&self) -> Constant {
        match self {
            BinaryExpression::Addition(expression) => expression.evaluate(),
            BinaryExpression::Subtraction(expression) => expression.evaluate(),
            BinaryExpression::Multiplication(expression) => expression.evaluate(),
            BinaryExpression::Division(expression) => expression.evaluate(),
            BinaryExpression::Modulus(expression) => expression.evaluate(),
            BinaryExpression::Exponent(expression) => expression.evaluate(),
            BinaryExpression::Root(expression) => expression.evaluate(),
            BinaryExpression::BitwiseAnd(expression) => expression.evaluate(),
            BinaryExpression::BitwiseOr(expression) => expression.evaluate(),
            BinaryExpression::BitwiseXor(expression) => expression.evaluate(),
            BinaryExpression::LeftShift(expression) => expression.evaluate(),
            BinaryExpression::RightShift(expression) => expression.evaluate(),
        }
    }
    pub fn new(
        left: Box<EnumExpression>,
        right: Box<EnumExpression>,
        operator: &str,
    ) -> BinaryExpression {
        match operator {
            "+" => BinaryExpression::Addition(AddExpression::new(left, right)),
            "-" => BinaryExpression::Subtraction(SubtractExpression::new(left, right)),
            "*" => BinaryExpression::Multiplication(MultiplyExpression::new(left, right)),
            "/" => BinaryExpression::Division(DivideExpression::new(left, right)),
            "%" => BinaryExpression::Modulus(ModuloExpression::new(left, right)),
            "&" => BinaryExpression::BitwiseAnd(AndExpression::new(left, right)),
            "|" => BinaryExpression::BitwiseOr(OrExpression::new(left, right)),
            "^" => BinaryExpression::BitwiseXor(XorExpression::new(left, right)),
            "**" => BinaryExpression::Exponent(ExponentExpression::new(left, right)),
            "//" => BinaryExpression::Root(RootExpression::new(left, right)),
            "<<" => BinaryExpression::LeftShift(LeftShiftExpression::new(left, right)),
            ">>" => BinaryExpression::RightShift(RightShiftExpression::new(left, right)),
            _ => panic!("Invalid binary operator"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NotExpression {
    pub expression: Box<EnumExpression>,
}

impl Expression for NotExpression {
    fn evaluate(&self) -> Constant {
        let expression = self.expression.evaluate();
        match expression {
            Constant::Integer(value) => Constant::Integer(!value),
            Constant::Binary(value) => Constant::Binary(!value),
            Constant::Hexadecimal(value) => Constant::Hexadecimal(!value),
            Constant::Octal(value) => Constant::Octal(!value),
            Constant::Float(value) => Constant::Integer(!(value as i64)),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Unary for NotExpression {
    fn new(expression: Box<EnumExpression>) -> Self {
        Self { expression }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NegateExpression {
    pub expression: Box<EnumExpression>,
}

impl Expression for NegateExpression {
    fn evaluate(&self) -> Constant {
        let expression = self.expression.evaluate();
        match expression {
            Constant::Integer(value) => Constant::Integer(-value),
            Constant::Binary(value) => Constant::Binary(-value),
            Constant::Hexadecimal(value) => Constant::Hexadecimal(-value),
            Constant::Octal(value) => Constant::Octal(-value),
            _ => panic!("Invalid addition operation"),
        }
    }
}

impl Unary for NegateExpression {
    fn new(expression: Box<EnumExpression>) -> Self {
        Self { expression }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UnaryExpression {
    Not(NotExpression),
    Negate(NegateExpression),
}

impl UnaryExpression {
    pub fn evaluate(&self) -> Constant {
        match self {
            UnaryExpression::Not(expression) => expression.evaluate(),
            UnaryExpression::Negate(expression) => expression.evaluate(),
        }
    }
    pub fn new(expression: Box<EnumExpression>, operator: &str) -> UnaryExpression {
        match operator {
            "!" => UnaryExpression::Not(NotExpression::new(expression)),
            "-" => UnaryExpression::Negate(NegateExpression::new(expression)),
            _ => panic!("Invalid unary operator"),
        }
    }
}

/**
 * EnumExpression is the enumerated version of the Expression trait.
    * It is used serialize and deserialize the Expression trait.
 */
#[derive(Deserialize, Serialize, Debug)]
pub enum EnumExpression {
    Constant(ConstExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
}

impl Expression for EnumExpression {
    fn evaluate(&self) -> Constant {
        match self {
            EnumExpression::Constant(expression) => expression.evaluate(),
            EnumExpression::Binary(expression) => expression.evaluate(),
            EnumExpression::Unary(expression) => expression.evaluate(),
        }
    }
}