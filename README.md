# simple_parser_rs
Basic math parser made with Rust, made for fun and for trying the language!

No thing to say, all rights (NOT) reserved

## Grammar!
This parser implement the following simple grammar

```
Digit            := '0' .. '9'
Comma            := '.'
Number           := [Sign] {Digit} [Comma {Digit}]
Sign             := '-'
OperatorPrio0    := '^'
OperatorPrio1    := '/' | '*' | '%'
OperatorPrio2    := '+' | '-'
LeftParenthesis  := '('
RightParenthesis := ')'
Invalid          := Every other symbol!

Term             := Number | LeftParenthesis Expression RightParenthesis
Power            := Term [ { OperatorPrio0 Term } ]
MultiplyDevide   := Power [ { OperatorPrio1 Power } ]
Expression       := MultiplyDevide [ { OperatorPrio2 MultiplyDevide } ]
```

It consists mainly of two modules, `lexer.rs` wich tokenize the input and `parser.rs` wich parse the tokens!, the `main.rs` implements a little console.
