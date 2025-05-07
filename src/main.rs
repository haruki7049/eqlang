use clap::Parser;
use eqlang::parser as eqlang_parser;
use eqlang::parser::Eqlang;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    let result: Eqlang = eqlang_parser::parse(&args.statement)?;

    dbg!(result.statements);

    Ok(())
}

#[derive(Debug, Parser)]
struct CLIArgs {
    statement: String,
}
