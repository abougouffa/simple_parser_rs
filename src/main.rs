extern crate simple_parser_rs;
use console::style;
use console::Term;

use simple_parser_rs::parser::*;

fn main() {
    let mut expr = String::new();

    println!(
        "Just a basic math parser made with Rust as a try to explore the language!
Type a math expression or type 'quit' to exit\n"
    );

    let stdout = Term::stdout();

    loop {
        stdout
            .write_str(format!("{}", style(">> ").cyan()).as_str())
            .ok();
        stdout.flush().ok();

        if let Ok(expr) = stdout.read_line() {
            let expr = expr.trim_end();
            if expr == "quit" {
                break;
            } else if let Ok(result) = Parser::parse(expr) {
                stdout
                    .write_str(format!("{}\n", style(format!(":: {}", result)).green()).as_str())
                    .ok();
            }
        }
        expr.clear();
    }
}
