use crate::logger::logger::{self, debug, error};
use super::{tokens::{Token}, expressions::{EnumExpression, ConstExpression, UnaryExpression, BinaryExpression, Expression}, constants::{Target, Constant}};
use logos::{Logos, Lexer};

#[allow(unused)]
const PRECEDENCE: &[(Token, u8)] = &[
    (Token::Exponent, 5),
    (Token::Root, 5),
    (Token::Multiply, 4),
    (Token::Divide, 4),
    (Token::Modulo, 4),
    (Token::Plus, 3),
    (Token::Minus, 3),
    (Token::LeftShift, 2),
    (Token::RightShift, 2),
    (Token::And, 1),
    (Token::Or, 1),
    (Token::Xor, 1),
];

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    // size: usize,
    // current: usize,
}


impl <'a> Parser <'a> {
    #[allow(unused)]
    pub fn new(string: &'a str) -> Self {
        let lexer = Token::lexer(string);
        let size = lexer.clone().spanned().count();
        Self {
            lexer,
            // size,
            // current: 0,
        }
    }

    /**
     Executes the parser
     * Parses the expression into an expression tree
     * Evaluates the expression tree
     */
    #[allow(unused)]
    pub fn execute(&mut self, t: Target) -> Result<Constant, String> {
        //find the first token
        //if the first token is a number or a unary operator, then insert it into the expression tree
        let expression = self.next_expression();
        if expression.is_none() {
            logger::error!("Could not parse the expression");
            return Err("Could not parse the expression".to_string());
        }
        let expression = expression.unwrap();
        let eval = expression.evaluate().into_target_constant(t);
        debug!("Evaluated expression: {:?}", serde_json::to_string(&eval).unwrap());
        Ok(eval)
    }
    pub fn next_value(&mut self) -> Option<EnumExpression> {
        #[allow(unused_assignments)]
        let mut expression: Option<EnumExpression> = None;
        let token = self.lexer.next();
        if token.is_none() {
            error!("Could not get the next token");
            return None;
        }
        let token = token.unwrap();
        if token.is_err() {
            let err = token.unwrap_err();
            error!("{:?}", err);
            return None;
        }
        let token = token.unwrap();
        match token {
            Token::Binary(_) | Token::Integer(_) | Token::Octal(_) | Token::Hexadecimal(_) | Token::Float(_) => {
                expression = Some(EnumExpression::Constant(ConstExpression::new(token)));
            },
            Token::Minus => {
                let next_expression = self.next_value();
                if next_expression.is_none() {
                    error!("Could not get the next expression");
                    return None;
                }
                expression = Some(EnumExpression::Unary(UnaryExpression::new(Box::new(next_expression.unwrap()), "-")));
            },
            Token::Not => {
                let next_expression = self.next_value();
                if next_expression.is_none() {
                    error!("Could not get the next expression");
                    return None;
                }
                expression = Some(EnumExpression::Unary(UnaryExpression::new(Box::new(next_expression.unwrap()), "!")));
            },
            Token::LeftParenthesis => {
                let next_expression = self.next_expression();
                if next_expression.is_none() {
                    error!("Could not get the next expression");
                    return None;
                }
                expression = Some(next_expression.unwrap());
            },
            _ => {
                error!("Could not parse the expression");
                return None;
            }
        }
        expression
    }
    pub fn next_expression(&mut self) -> Option<EnumExpression> {
        #[allow(unused_assignments)]
        let mut expression: Option<EnumExpression> = None;
        let left = self.next_value();
        if left.is_none() {
            error!("Could not get the next value");
            return None;
        }
        let next = self.lexer.next();
        if next.is_none() {
            return left;
        }
        let next = next.unwrap();
        if next.is_err() {
            return left;
        }
        let next = next.unwrap();
        if next == Token::RightParenthesis {
            return left;
        }
        let right = self.next_expression();
        match next {
            Token::Exponent => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "**")));
            },
            Token::Root => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "//")));
            },
            Token::Multiply => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "*")));
            },
            Token::Divide => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "/")));
            },
            Token::Modulo => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "%")));
            },
            Token::Plus => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "+")));
            },
            Token::Minus => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "-")));
            },
            Token::LeftShift => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "<<")));
            },
            Token::RightShift => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), ">>")));
            },
            Token::And => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "&")));
            },
            Token::Or => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "|")));
            },
            Token::Xor => {
                expression = Some(EnumExpression::Binary(BinaryExpression::new(Box::new(left.unwrap()), Box::new(right.unwrap()), "^")));
            },
            _ => {
                error!("Could not parse the expression");
                return None;
            }
        }
        expression
    }
}