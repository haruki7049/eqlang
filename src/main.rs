use clap::Parser;
use eqlang::parser as eqlang_parser;
use eqlang::parser::{Eqlang, Statement};

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    let mut result: Eqlang = eqlang_parser::parse(&args.statement)?;

    eval(&mut result)?;

    Ok(())
}

/// Evaluates statements
fn eval(eqlang: &mut Eqlang) -> anyhow::Result<()> {
    eqlang.statements.reverse();

    // Get the values
    let cmd: Statement = eqlang.statements.pop().unwrap();
    let first_argument: Statement = eqlang.statements.pop().unwrap();
    let second_argument: Statement = eqlang.statements.pop().unwrap();

    // Checks whether cmd is a command
    if !cmd.is_command() {
        panic!("EVAL ERROR: You must write a command, as '!!?!?!??' <- Plus");
    }

    // Addition
    if cmd == Statement::Plus {
        println!("{}", first_argument + second_argument);
    }

    Ok(())
}

#[derive(Debug, Parser)]
struct CLIArgs {
    statement: String,
}
