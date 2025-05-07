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

impl Statement {
    /// Whether the statement is a command or not
    pub fn is_command(self) -> bool {
        self == Statement::Plus
    }
}

impl std::ops::Add for Statement {
    type Output = usize;

    fn add(self, other: Self) -> Self::Output {
        let first_number: Number = match self {
            Statement::Plus => panic!(),
            Statement::Number(v) => v,
        };
        let second_number: Number = match other {
            Statement::Plus => panic!(),
            Statement::Number(v) => v,
        };

        first_number + second_number
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Statement {
    Plus,
    Number(Number),
}

impl std::ops::Add for Number {
    type Output = usize;

    fn add(self, other: Self) -> Self::Output {
        let first_value: usize = match self {
            Number::Zero => 0,
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
        };
        let second_value: usize = match other {
            Number::Zero => 0,
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
        };

        first_value + second_value
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
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

                    // +
                    "!!?!?!??" => result.statements.push(Statement::Plus),

                    // 0~9
                    "?????!!?" => result.statements.push(Statement::Number(Number::Zero)),
                    "???!?!!?" => result.statements.push(Statement::Number(Number::One)),
                    "??!??!!?" => result.statements.push(Statement::Number(Number::Two)),
                    "??!!?!!?" => result.statements.push(Statement::Number(Number::Three)),
                    "?!???!!?" => result.statements.push(Statement::Number(Number::Four)),
                    "?!?!?!!?" => result.statements.push(Statement::Number(Number::Five)),
                    "?!!??!!?" => result.statements.push(Statement::Number(Number::Six)),
                    "?!!!?!!?" => result.statements.push(Statement::Number(Number::Seven)),
                    "!????!!?" => result.statements.push(Statement::Number(Number::Eight)),
                    "!??!?!!?" => result.statements.push(Statement::Number(Number::Nine)),

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
