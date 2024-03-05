use std::{error::Error, fmt::Display}; 

use super::tokenizer::{Token, Tokenizer, TokenizerError};

#[allow(dead_code)]
pub struct ParserV2<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> ParserV2<'a> {
    pub fn new(expression: &'a str) -> Result<Self, ParserError> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = if let Some(Ok(token)) = tokenizer.next() {
            token
        } else {
            return Err(ParserError::EmptyOrInvalidExpression);
        };

        Ok(Self {
            tokenizer,
            current_token,
        })
    }

    pub fn parse(&mut self) -> Result<NodeV2, ParserError> {
        self.parse_add_sub()
    }

    pub fn parse_add_sub(&mut self) -> Result<NodeV2, ParserError> {
        let left = self.parse_mul_div_pow()?;
        match self.current_token {
            Token::Add | Token::Subtract => {
                let operator = if self.current_token == Token::Add {
                    BinaryOperator::Add
                } else {
                    BinaryOperator::Sub
                };
                self.advance_token()?;
                let right = self.parse_add_sub()?;
                Ok(NodeV2::BinaryNode(
                    Box::new(left),
                    operator,
                    Box::new(right),
                ))
            },
            Token::End => Ok(left),
            _ => Ok(left),
            // _ => return Err(ParserError::InvalidStateReached(self.current_token)),
        }
    }

    pub fn parse_mul_div_pow(&mut self) -> Result<NodeV2, ParserError> {
        let left = self.parse_num_neg_paren()?;
        match self.current_token {
            Token::Multiply | Token::Divide | Token::Power => {
                let operator = match self.current_token {
                    Token::Multiply => BinaryOperator::Mul,
                    Token::Divide => BinaryOperator::Div,
                    Token::Power => BinaryOperator::Pow,
                    _ => unreachable!(),
                };
                self.advance_token()?;
                let right = self.parse_num_neg_paren()?;
                // let right = self.parse_add_sub()?;
                Ok(NodeV2::BinaryNode(Box::new(left), operator, Box::new(right)))
            }
            _ => Ok(left),
        }
    }

    pub fn parse_num_neg_paren(&mut self) -> Result<NodeV2, ParserError> {
        match self.current_token {
            Token::Number(value) => {
                self.advance_token()?;
                Ok(NodeV2::NumberNode(value))
            },
            Token::Subtract => {
                self.advance_token()?;
                // let negative_expression = self.parse_add_sub()?;
                let negative_expression = self.parse_num_neg_paren()?;

                Ok(NodeV2::UnaryNode(UnaryOperator::Neg, Box::new(negative_expression)))
            },
            Token::OpenParenthesis => {
                self.advance_token()?;
                let inner_expression = self.parse_add_sub()?;
                // Check for the closing parenthesis!
                if self.current_token != Token::CloseParenthesis {
                    return Err(ParserError::UnmatchedParenthesism);
                }
                Ok(inner_expression)
            },
            _ => Err(ParserError::InvalidStateReached(self.current_token))
        }
    }

    fn advance_token(&mut self) -> Result<(), ParserError> {
        self.current_token = match self.tokenizer.next() {
            Some(Ok(token)) => token,
            None => todo!(), // ???
            Some(Err(e)) => return Err(ParserError::WrappedTokenizerError(e)),
        };
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum NodeV2 {
    UnaryNode(UnaryOperator, Box<NodeV2>),
    BinaryNode(Box<NodeV2>, BinaryOperator, Box<NodeV2>),
    NumberNode(f64),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum UnaryOperator {
    Neg,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserError {
    EmptyOrInvalidExpression,
    InvalidStateReached(Token),
    WrappedTokenizerError(TokenizerError),
    UnmatchedParenthesism
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ParserError {}
