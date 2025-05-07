use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "eqlang.pest"]
pub struct EqlangParser;

#[derive(Debug, Default)]
pub struct Eqlang {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Plus,
    Number(Number),
}

#[derive(Debug)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Eqlang> {
    match pair.as_rule() {
        Rule::EOI | Rule::word | Rule::character | Rule::PUNCT_WORD => unreachable!(),
        Rule::statement => {
            let mut rule = pair.into_inner();
            let mut statement = rule.next().unwrap();
            let mut result: Eqlang = Eqlang::default();

            // Now `statement` has a word

            while let s = statement.as_span().as_str() {
                match s {
                    "" => break,
                    "!!?!?!??" => result.statements.push(Statement::Plus),
                    e => panic!("PARSE ERROR: Unknown word, {}", e),
                }

                // Update statement to next word
                statement = rule.next().unwrap();
            }

            Ok(result)
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<Eqlang> {
    let mut pairs: Pairs<Rule> = EqlangParser::parse(Rule::statement, s)?;

    Ok(parse_pair(pairs.next().unwrap())?)
}
