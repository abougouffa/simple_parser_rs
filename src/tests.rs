#[cfg(test)]
use super::parser::*;

#[test]
fn simple_numbers() {
    assert!(Parser::parse("123") == Ok(123.));
    assert!(Parser::parse("1+2+3") == Ok(6.));
    assert!(Parser::parse("1*2*3*4*5") == Ok(120.));
    assert!(Parser::parse("((((((66667))))))") == Ok(66667.));
    assert!(Parser::parse("(((((((((((((((4)+5)))))))-2))))--1)))") == Ok(8.));
}

#[test]
fn complex() {
    assert!(Parser::parse("(12)*(1.5)/6") == Ok(3.));
    assert!(Parser::parse("(12)*(1.5) * 98^6") == Ok(15945162855552.0));
    assert!(Parser::parse("0-1") == Ok(-1.));
    assert!(Parser::parse("0--1") == Ok(1.));
    assert!(Parser::parse("0---1") == Ok(-1.));
    assert!(Parser::parse("0----1") == Ok(1.));

    // this will fail (not implemented yet)
    // assert!(Parser::parse("1*-11") == Ok(-11.));
    // assert!(Parser::parse("-1") == Ok(-1.));
}

#[test]
fn errors() {
    assert!(Parser::parse("X") == Err("WTF2! Got an Invalid token!"));
    assert!(Parser::parse("X") == Err("WTF2! Got an Invalid token!"));
}
