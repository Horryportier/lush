use std::{env, ffi::OsString, fmt::Display, io::{self, Write}};

use crossterm::style::{ContentStyle, Stylize};

use crate::LushError;

pub type P = Promptable;

pub enum Promptable {
    Env(String, ContentStyle),
    Str(String, ContentStyle),
}

impl Display for Promptable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let env = match self {
            Self::Str(s, style) => style.apply(s.clone()),
            Self::Env(s, style) => match env::var(OsString::from(s)) {
                Ok(s) => style.apply(s),
                Err(e) => e.to_string().red(),
            },
        };
        write!(f, "{env}")
    }
}

#[derive()]
pub struct Prompt {
    pub lines: Vec<(Vec<P>, String)>,
}

impl Prompt {
   pub fn render(&self) -> Result<(), LushError> {
        print!("{}", self);
        Ok(io::stdout().flush()?)
   } 
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .lines
            .iter()
            .map(|x| {
                x.0.iter()
                    .map(|y| y.to_string())
                    .collect::<Vec<String>>()
                    .join(&x.1)
            })
            .collect();

        write!(f, "{}", lines.join("\n"))
    }
}
