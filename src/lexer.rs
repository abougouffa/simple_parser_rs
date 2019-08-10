use console::style;
#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo,
}

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Start,
    Number(f64),
    Operator(Operator),
    Invalid,
    Ignore,
    LeftParenthesis,
    RightParenthesis,
    Sign(f64),
    Comma,
    End,
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    current_index: usize,
}

impl Lexer {
    pub fn new(expression: &str) -> Lexer {
        let mut token;
        let mut space_seen_before = false;
        let mut divider = 1.;
        let mut lex = Lexer {
            tokens: vec![Token::Start],
            current_index: 0,
        };

        for c in expression.chars() {
            token = match c {
                '+' => Token::Operator(Operator::Plus),
                '-' => Token::Operator(Operator::Minus),
                '*' => Token::Operator(Operator::Multiply),
                '/' => Token::Operator(Operator::Divide),
                '%' => Token::Operator(Operator::Modulo),
                '^' => Token::Operator(Operator::Power),
                '(' => Token::LeftParenthesis,
                ')' => Token::RightParenthesis,
                '.' => {
                    divider = 10.;
                    Token::Comma
                }
                ' ' | '\n' | '\t' => {
                    space_seen_before = true;
                    Token::Ignore
                }
                _ if c.is_digit(10) => Token::Number(c.to_digit(10).unwrap().into()),
                _ => Token::Invalid,
            };

            if let Token::Ignore | Token::Comma = token {
                continue;
            }

            let last_index = lex.tokens.len() - 1;
            let previous_token = &mut lex.tokens[last_index];

            match token {
                Token::Number(num) => {
                    if let Token::Number(pnum) = &previous_token {
                        if space_seen_before {
                            // two numbers with space between => syntax error
                            println!(
                                "{}",
                                style(format!("!! Lexer Error: Illegal space after '{}'", pnum))
                                    .red()
                            );
                            return Lexer {
                                tokens: vec![Token::Invalid],
                                current_index: 0,
                            };
                        }

                        if divider > 1. {
                            *previous_token = Token::Number(pnum + num / divider);
                            divider *= 10.;
                        } else {
                            *previous_token = Token::Number(pnum * 10. + num);
                        }
                        continue;
                    } else if let Token::Sign(s) = &previous_token {
                        *previous_token = Token::Number(s * num);
                        continue;
                    }
                }
                Token::Operator(op) => {
                    if let Token::Operator(pop) = &previous_token {
                        if let Operator::Minus = op {
                            match pop {
                                Operator::Plus => {
                                    *previous_token = Token::Operator(Operator::Minus)
                                }
                                Operator::Minus => {
                                    *previous_token = Token::Operator(Operator::Plus)
                                }
                                _ => *previous_token = Token::Sign(-1.),
                            }
                            continue;
                        } else {
                            println!(
                                "{}",
                                style(format!(
                                    "!! Lexer Error: Illegal order of operators '{}' and before!",
                                    c
                                ))
                                .red()
                            );
                            return Lexer {
                                tokens: vec![Token::Invalid],
                                current_index: 0,
                            };
                        }
                    }
                }
                Token::Invalid => {
                    println!(
                        "{}",
                        style(format!("!! Lexer Error: Invalid token: '{}'", c)).red()
                    );
                    return Lexer {
                        tokens: vec![Token::Invalid],
                        current_index: 0,
                    };
                }
                _ => {}
            }

            lex.tokens.push(token);
            space_seen_before = false;
            divider = 1.;
        }

        lex.tokens.push(Token::End);
        lex
    }

    pub fn next_token(&mut self) -> Token {
        self.current_index += 1;
        if self.current_index < self.tokens.len() {
            self.tokens[self.current_index]
        } else {
            Token::Invalid
        }
    }

    pub fn prev_token(&mut self) -> Token {
        if self.current_index != 0 {
            self.current_index -= 1;
            self.tokens[self.current_index + 1]
        } else {
            Token::Invalid
        }
    }
}
