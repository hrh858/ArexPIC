use crate::core::tokenizer::{Token, Tokenizer};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),      // Add two nodes
    Subtract(Box<Node>, Box<Node>), // Subtract two nodes
    Multiply(Box<Node>, Box<Node>), // Multiply two nodes
    Divide(Box<Node>, Box<Node>),   // Divide two nodes
    Power(Box<Node>, Box<Node>),    // Power a node to an exponent
    Negative(Box<Node>),            // Make a node negative
    Number(f64),
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> Result<Self, Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = match tokenizer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(Box::new(e)),
            None => return Err(Box::new(ParserError::NoTokens)),
        };
        Ok(Self {
            tokenizer,
            current_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, Box<dyn Error>> {
        self.parse_rec(OperationPrecedence::DefaultZero)
    }

    fn parse_rec(&mut self, op_prec: OperationPrecedence) -> Result<Node, Box<dyn Error>> {
        let mut left = self.process_number()?;
        while op_prec < self.current_token.get_operation_precedence() {
            if self.current_token == Token::End {
                break;
            }
            let right = self.process_node(left.clone())?;
            left = right
        }
        Ok(left)
    }

    fn process_node(&mut self, left: Node) -> Result<Node, Box<dyn Error>> {
        match self.current_token {
            Token::Add => {
                self.advance_token()?;
                let right = self.parse_rec(OperationPrecedence::AddSubtract)?;
                Ok(Node::Add(Box::new(left), Box::new(right)))
            },
            Token::Subtract => {
                self.advance_token()?;
                let right = self.parse_rec(OperationPrecedence::AddSubtract)?;
                Ok(Node::Subtract(Box::new(left), Box::new(right)))
            },
            Token::Multiply => {
                self.advance_token()?;
                let right = self.parse_rec(OperationPrecedence::MultiplyDivision)?;
                Ok(Node::Multiply(Box::new(left), Box::new(right)))
            },
            Token::Divide => {
                self.advance_token()?;
                let right = self.parse_rec(OperationPrecedence::MultiplyDivision)?;
                Ok(Node::Divide(Box::new(left), Box::new(right)))
            },
            Token::Power => {
                self.advance_token()?;
                let right = self.parse_rec(OperationPrecedence::Power)?;
                Ok(Node::Power(Box::new(left), Box::new(right)))
            },
            _ => todo!()
        }
    }

    fn process_number(&mut self) -> Result<Node, Box<dyn Error>> {
        match self.current_token {
            Token::Subtract => {
                self.advance_token()?;
                let inner = self.parse_rec(OperationPrecedence::Negative)?;
                Ok(Node::Negative(Box::new(inner)))
            },
            Token::Number(value) => {
                self.advance_token()?;
                Ok(Node::Number(value))
            },
            Token::OpenParenthesis => {
                self.advance_token()?;
                let inner = self.parse_rec(OperationPrecedence::DefaultZero)?;
                self.check_parenthesis()?;
                if self.current_token == Token::OpenParenthesis {
                    let right  = self.parse_rec(OperationPrecedence::MultiplyDivision)?;
                    return Ok(Node::Multiply(Box::new(inner), Box::new(right)));
                }
                Ok(inner)
            },
            _ => Err(Box::new(ParserError::InvalidExpression))
        }
    }

    fn advance_token(&mut self) -> Result<(), Box<dyn Error>> {
        let next_token = match self.tokenizer.next() {
            Some(Ok(t)) => t,
            Some(Err(e)) => return Err(Box::new(e)),
            None => return Err(Box::new(ParserError::NoTokens))
        };
        self.current_token = next_token;
        Ok(())
    }

    fn check_parenthesis(&mut self) -> Result<(), Box<dyn Error>> {
        if self.current_token == Token::CloseParenthesis {
            self.advance_token()?;
            Ok(())
        } else {
            Err(Box::new(ParserError::UnmatchedParenthesis))
       }
    }

}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperationPrecedence {
    DefaultZero,
    AddSubtract,
    MultiplyDivision,
    Power,
    Negative,
}

#[derive(Debug)]
pub enum ParserError {
    NoTokens,
    UnmatchedParenthesis,
    InvalidExpression,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTokens => write!(f, "no tokens could be extracted from the expression"),
            Self::UnmatchedParenthesis => write!(f, "there are unmatched parenthesis in the expression"),
            Self::InvalidExpression => write!(f, "the expression isn't valid"),
        }
    }
}

impl Error for ParserError {}
