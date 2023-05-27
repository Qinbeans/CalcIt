use crate::logger::logger::{self, debug, error};
use super::{tokens::{Token}, expressions::{EnumExpression, ConstExpression, UnaryExpression, BinaryExpression, Expression}, constants::{Target, Constant}};
use logos::{Logos, Lexer};

/**
 A parser that takes a string and returns a constant.
 */
pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    current: usize,
    size: usize,
}

impl <'a> Parser <'a> {
    pub fn new(string: &'a str) -> Self {
        debug!("Original: {}", string);
        let precedence = Self::gen_precedence(string);
        let lexer = Token::lexer(precedence);
        let size = lexer.clone().spanned().count();
        Self {
            lexer,
            current: 0,
            size,
        }
    }

    /**
     Generates a string with parenthesis to enforce operator precedence.
     
     Inspired by: https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method
     */
    pub fn gen_precedence(string: &str) -> &'static str {
        let mut lexer = Token::lexer(string);
        let mut precedence = "((((((((".to_string();
        let mut next = lexer.next();
        let mut prev: Option<Token> = None;
        while let Some(r_token) = next.clone() {
            if let Ok(token) = r_token {
                match token {
                    Token::Binary(b) => {
                        precedence.push_str(&format!("{}", b));
                    },
                    Token::Integer(i) => {
                        precedence.push_str(&format!("{}", i));
                    },
                    Token::Octal(o) => {
                        precedence.push_str(&format!("{}", o));
                    },
                    Token::Hexadecimal(h) => {
                        precedence.push_str(&format!("{}", h));
                    },
                    Token::Float(f) => {
                        precedence.push_str(&format!("{}", f));
                    },
                    Token::Minus => {
                        if prev.is_none() || !prev.as_ref().unwrap().is_number() {
                            precedence.push_str("-");
                        } else {
                            precedence.push_str("))))-((((");
                        }
                    },
                    Token::Not => {
                        precedence.push_str("!");
                    },
                    Token::LeftParenthesis => {
                        precedence.push_str("((((((((");
                    },
                    Token::RightParenthesis => {
                        precedence.push_str("))))))))");
                    },
                    Token::Exponent => {
                        precedence.push_str(")**(");
                    },
                    Token::Root => {
                        precedence.push_str(")//(");
                    },
                    Token::Multiply => {
                        precedence.push_str("))*((");
                    },
                    Token::Divide => {
                        precedence.push_str("))/((");
                    },
                    Token::Modulo => {
                        precedence.push_str(")))%(((");
                    },
                    Token::Plus => {
                        precedence.push_str("))))+((((");
                    },
                    Token::LeftShift => {
                        precedence.push_str(")))))<<(((((");
                    },
                    Token::RightShift => {
                        precedence.push_str(")))))>>(((((");
                    },
                    Token::And => {
                        precedence.push_str("))))))&((((((");
                    },
                    Token::Or => {
                        precedence.push_str("))))))|((((((");
                    },
                    Token::Xor => {
                        precedence.push_str(")))))))^(((((((");
                    },
                }
                prev = Some(token);
                next = lexer.next();
            }
        }
        precedence.push_str("))))))))");
        debug!("Precedence: {}", precedence);
        Box::leak(precedence.into_boxed_str())
    }

    /**
     Executes the parser
     * Parses the expression into an expression tree
     * Evaluates the expression tree
     */
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
    /**
     Creates constant expressions from the tokens
     */
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
        self.current += 1;
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
                if self.current >= self.size && self.lexer.slice() != ")" {
                    error!("Could not find right parenthesis");
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
    /**
     Recursively finds the next expression
     */
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
        self.current += 1;
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