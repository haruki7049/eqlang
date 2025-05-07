use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "eqlang.pest"]
pub struct EqlangParser;

#[derive(Debug)]
pub struct Statement {}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Statement> {
    match pair.as_rule() {
        Rule::EOI | Rule::word | Rule::character | Rule::PUNCT_WORD => unreachable!(),
        Rule::statement => {
            let mut rule = pair.into_inner();

            loop {
                let statement = rule.next().unwrap();
                dbg!(statement);
            }

            todo!()
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<Statement> {
    let mut pairs: Pairs<Rule> = EqlangParser::parse(Rule::statement, s)?;

    Ok(parse_pair(pairs.next().unwrap())?)
}
