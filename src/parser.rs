/**
 * ::::::::::::: GRAMMAR :::::::::::::::
 * Digit            := '0' .. '9'
 * Comma            := '.'
 * Number           := [Sign] {Digit} [Comma {Digit}]
 * Sign             := '-'
 * OperatorPrio0    := '^'
 * OperatorPrio1    := '/' | '*' | '%'
 * OperatorPrio2    := '+' | '-'
 * LeftParenthesis  := '('
 * RightParenthesis := ')'
 * Invalid          := Every other symbol!
 *
 * Term             := Number | LeftParenthesis Expression RightParenthesis
 * Power            := Term [ { OperatorPrio0 Term } ]
 * MultiplyDevide   := Power [ { OperatorPrio1 Power } ]
 * Expression       := MultiplyDevide [ { OperatorPrio2 MultiplyDevide } ]
 */
use super::lexer::*;
use console::style;

pub struct Parser {}

impl Parser {
    pub fn parse(exp: &str) -> Result<f64, &str> {
        let mut lex = Lexer::new(exp);
        if let Token::Invalid = lex.next_token() {
            Err("Parser Error: Got an Invalid token!")
        } else {
            let _ = lex.prev_token();
            let expr = Parser::expression(&mut lex);

            if let Ok(result) = expr {
                let end_token = lex.next_token();

                if let Token::End = end_token {
                    Ok(result)
                } else {
                    Err("Parser Error: Expected end of token, This error may never occur!")
                }
            } else {
                Err("Parser Error: Check your f*****g expression")
            }
        }
    }

    fn term(lex: &mut Lexer) -> Result<f64, Token> {
        let token = lex.next_token();
        match token {
            Token::LeftParenthesis => {
                let expr = Parser::expression(lex)?;
                if let Token::RightParenthesis = lex.next_token() {
                    return Ok(expr);
                } else {
                    println!("{}", style("!! Parser Error: Unclosed parenthesis!").red());
                }
            }
            Token::Number(num) => return Ok(num),
            _ => {}
        }
        Err(token)
    }

    fn power(lex: &mut Lexer) -> Result<f64, Token> {
        let mut operand1 = Parser::term(lex)?;
        let mut operator = lex.next_token();

        while match operator {
            Token::Operator(Operator::Power) => true,
            _ => false,
        } {
            let operand2 = Parser::term(lex)?;
            match operator {
                Token::Operator(Operator::Power) => {
                    operand1 = f64::powi(operand1, operand2 as i32);
                }
                _ => {}
            }
            operator = lex.next_token();
        }
        let _ = lex.prev_token();
        Ok(operand1)
    }

    fn mul_dev(lex: &mut Lexer) -> Result<f64, Token> {
        let mut operand1 = Parser::power(lex)?;
        let mut operator = lex.next_token();

        while match operator {
            Token::Operator(Operator::Multiply)
            | Token::Operator(Operator::Divide)
            | Token::Operator(Operator::Modulo) => true,
            _ => false,
        } {
            let operand2 = Parser::power(lex)?;
            match operator {
                Token::Operator(Operator::Multiply) => {
                    operand1 = operand1 * operand2;
                }
                Token::Operator(Operator::Divide) => {
                    operand1 = operand1 / operand2;
                }
                Token::Operator(Operator::Modulo) => {
                    if (((operand1 as i64) as f64) - operand1 == 0.)
                        && (((operand2 as i64) as f64) - operand2 == 0.)
                    {
                        operand1 = (operand1 as i64 % operand2 as i64) as f64;
                    } else {
                        println!("{}", style("!! Parser Error: Can't calculate modulos on floating point numbers!").red());
                        return Err(operator);
                    }
                }
                _ => {}
            }
            operator = lex.next_token();
        }
        let _ = lex.prev_token();
        Ok(operand1)
    }

    fn expression(lex: &mut Lexer) -> Result<f64, Token> {
        let mut operand1 = Parser::mul_dev(lex)?;
        let mut operator = lex.next_token();

        while match operator {
            Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) => true,
            _ => false,
        } {
            let operand2 = Parser::mul_dev(lex)?;
            match operator {
                Token::Operator(Operator::Plus) => {
                    operand1 = operand1 + operand2;
                }
                Token::Operator(Operator::Minus) => {
                    operand1 = operand1 - operand2;
                }
                _ => {}
            }
            operator = lex.next_token();
        }
        let _ = lex.prev_token();
        Ok(operand1)
    }
}
