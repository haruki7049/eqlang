use clap::Parser;
use eqlang::parser as eqlang_parser;
use eqlang::parser::Statement;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    let statement: Statement = eqlang_parser::parse(&args.statement)?;
    dbg!(statement);

    Ok(())
}

#[derive(Debug, Parser)]
struct CLIArgs {
    statement: String,
}
