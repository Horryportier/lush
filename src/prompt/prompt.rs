use std::{env, ffi::OsString, fmt::Display};

use colored::Colorize;


pub type P = Promptable;

#[derive(Debug, Clone)]
pub enum Promptable {
    Env(String),
    Str(String),
}

impl Display for Promptable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let env = match self {
            Self::Str(s) => s.clone().bold().yellow(),
            Self::Env(s) => match env::var(OsString::from(s)) {
                Ok(s) => s.green(),
                Err(e) => e.to_string().red(),
            },
        };
        write!(f, "{env}")
    }
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub lines: Vec<(Vec<P>, String)>,
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
