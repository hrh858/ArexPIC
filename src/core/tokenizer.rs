use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

use super::parser::OperationPrecedence;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    expression: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        Tokenizer {
            // We want to iterate on the characters of the input and also be able to peek into the
            // next character (helpful to parse numbers).
            expression: expression.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(exp_char) = self.expression.next() {
            // Let's ignore spaces so the user can do `1 + 1` not just `1+1` by recursively calling
            // this same function.
            if exp_char == ' ' {
                self.next()
            } else {
                match exp_char {
                    // These symbols are straight forward.
                    '+' => Some(Ok(Token::Add)),
                    '-' => Some(Ok(Token::Subtract)),
                    '*' => Some(Ok(Token::Multiply)),
                    '/' => Some(Ok(Token::Divide)),
                    '^' => Some(Ok(Token::Power)),
                    '(' => Some(Ok(Token::OpenParenthesis)),
                    ')' => Some(Ok(Token::CloseParenthesis)),
                    // Parsing a number, not so much...
                    '0'..='9' => {
                        // Start a string with the digit we just read.
                        let mut number_expr = String::from(exp_char);
                        // Check if the next char in the expression is a digit or a `.` (decimal
                        // values).
                        while let Some(next_expr_char) = self.expression.peek() {
                            if next_expr_char.is_ascii_digit() || *next_expr_char == '.' {
                                // If it is, add it to the string.
                                number_expr.push(self.expression.next().unwrap())
                            } else {
                                // if it isn't then break.
                                break;
                            }
                        }
                        // Try to parse the string into a float or error
                        if let Ok(val) = number_expr.parse() {
                            Some(Ok(Token::Number(val)))
                        } else {
                            // I think this should never happen after the previous checks, but... let's add it just in
                            // case lol
                            Some(Err(TokenizerError::ErrorParsingNumber(number_expr)))
                        }
                    }
                    // All the other characters aren't supported so error if we find one.
                    _ => Some(Err(TokenizerError::InvalidCharacter(exp_char))),
                }
            }
        } else {
            Some(Ok(Token::End))
        }
    }
}

#[derive(Debug)]
pub enum TokenizerError {
    // Error generated a string from the expression that should parse into a float cannot be converted.
    ErrorParsingNumber(String),
    // Error generated when an invalid character (not supported is also invalid :P) is found in the expression.
    InvalidCharacter(char),
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::ErrorParsingNumber(number_str) => write!(
                f,
                "an error occurred when trying to convert '{}' into a number",
                number_str
            ),
            TokenizerError::InvalidCharacter(character) => write!(
                f,
                "found an invalid (or not supported) character {}",
                character
            ),
        }
    }
}

impl Error for TokenizerError {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Add,              // The `+` symbol
    Subtract,         // The `-` symbol
    Multiply,         // The `*` symbol
    Divide,           // The `/` symbol
    Power,            // The `^` symbol
    OpenParenthesis,  // The `(` symbol
    CloseParenthesis, // The `)` symbol
    Number(f64),      // Any number, eg: 3.141 or 42
    End
}

impl Token {
    pub fn get_operation_precedence(&self)  -> OperationPrecedence {
        match *self {
            Self::Add | Self:: Subtract => OperationPrecedence::AddSubtract,
            Self::Multiply | Self::Divide => OperationPrecedence::MultiplyDivision,
            Self::Power => OperationPrecedence::Power,
            _ => OperationPrecedence::DefaultZero,
        }
    }
}

#[test]
fn test_tokenizer_happy_path() {
    let input: &str = "3.23 + 10.0993 - ((3*2)/24) ^ 2"; // This input should cover all the
                                                         // cases... I hope!
    let expected_output: [Token; 16] = [
        Token::Number(3.23),
        Token::Add,
        Token::Number(10.0993),
        Token::Subtract,
        Token::OpenParenthesis,
        Token::OpenParenthesis,
        Token::Number(3.0),
        Token::Multiply,
        Token::Number(2.0),
        Token::CloseParenthesis,
        Token::Divide,
        Token::Number(24.0),
        Token::CloseParenthesis,
        Token::Power,
        Token::Number(2.0),
        Token::End,
    ];
    let mut t = Tokenizer::new(input).enumerate();
    while let Some((idx, Ok(token))) = t.next() {
        assert_eq!(token, expected_output[idx])
    }
}

#[test]
fn test_tokenizer_invalid_char_error() {
    let input: &str = "cos(90)"; // This input should cover all the
                                 // cases... I hope!
    let mut t = Tokenizer::new(input);
    if let Some(res) = t.next() {
        match res {
            Ok(_) => panic!("error expected"),
            Err(err) => match err {
                TokenizerError::InvalidCharacter(_) => {}
                _ => panic!("this is not the expected error"),
            },
        }
    }
}
