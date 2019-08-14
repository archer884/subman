use crate::ini;
use pest::Parser;
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "../resource/ini.pest"]
pub struct IniParser;

#[derive(Clone, Debug)]
pub enum ParseEvent<'input> {
    Section(&'input str),
    Key(&'input str),
    Value(&'input str),
}

impl<'a> ParseEvent<'a> {
    pub fn into_str(self) -> &'a str {
        match self {
            ParseEvent::Section(s) => s.trim_matches(|u| u == '[' || u == ']'),
            ParseEvent::Key(s) => s,
            ParseEvent::Value (s) => s,
        }
    }
}

impl IniParser {
    pub fn parse_from_str<'input>(
        s: &'input str,
    ) -> ini::Result<impl Iterator<Item = ParseEvent> + 'input> {
        Ok(IniParser::parse(Rule::File, s)?
            .flatten()
            .filter_map(|x| match x.as_rule() {
                Rule::Section => Some(ParseEvent::Section(dbg!(x.as_str()))),
                Rule::Key => Some(ParseEvent::Key(x.as_str())),
                Rule::Value => Some(ParseEvent::Value(x.as_str())),

                // Uninteresting parse events will not be produced
                _ => None, 
            }))
    }
}
