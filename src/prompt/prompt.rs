use std::{
    env,
    ffi::OsString,
    fmt::Display,
    io::{self, Write},
};

use crossterm::style::{ContentStyle, Stylize};

use crate::{colors::colors::DEF_STYLE, LushError};

pub type P = Promptable;

pub const DEF_SPLIT: fn() -> P = || P::Str(String::from(" "), DEF_STYLE());

pub enum Promptable {
    Env(String, ContentStyle),
    Str(String, ContentStyle),
    Fun(fn() -> String, ContentStyle)
}

impl Display for Promptable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let env = match self {
            Self::Str(s, style) => style.apply(s.clone()),
            Self::Env(s, style) => match env::var(OsString::from(s)) {
                Ok(s) => style.apply(s),
                Err(e) => e.to_string().red(),
            },
            Self::Fun(f,style) => style.apply(f())
        };
        write!(f, "{env}")
    }
}

#[derive()]
pub struct Prompt {
    pub lines: Vec<(Vec<P>, P)>,
}

impl Prompt {
    #[allow(dead_code)]
    pub fn new() -> Prompt {
        Prompt { lines: Vec::new() }
    }
    pub fn add_line(mut self, split: P) -> Prompt {
        self.lines.append(&mut vec![(Vec::new(), split)]);
        self
    }

    pub fn add_str(mut self, string: String, style: ContentStyle) -> Prompt {
        match self.lines.last_mut() {
            Some(l) => {
                l.0.append(&mut vec![P::Str(string, style)]);
                return self;
            }
            None => {
                self = self.add_line(DEF_SPLIT());
                self.lines
                    .last_mut()
                    .unwrap()
                    .0
                    .append(&mut vec![P::Str(string, style)]);
                return self;
            }
        }
    }

    pub fn add_env(mut self, env: String, style: ContentStyle) -> Prompt {
        match self.lines.last_mut() {
            Some(l) => {
                l.0.append(&mut vec![P::Env(env, style)]);
                return self;
            }
            None => {
                self = self.add_line(DEF_SPLIT());
                self.lines
                    .last_mut()
                    .unwrap()
                    .0
                    .append(&mut vec![P::Env(env, style)]);
                return self;
            }
        }
    }

    pub fn add_fun(mut self, fun:  fn() -> String, style: ContentStyle) -> Prompt {
        match self.lines.last_mut() {
            Some(l) => {
                l.0.append(&mut vec![P::Fun(fun, style)]);
                return self;
            }
            None => {
                self = self.add_line(DEF_SPLIT());
                self.lines
                    .last_mut()
                    .unwrap()
                    .0
                    .append(&mut vec![P::Fun(fun, style)]);
                return self;
            }
        }
    }
    pub fn default() -> Prompt {
        let tmp = Prompt { lines: Vec::new() };
        tmp.add_line(P::Str("::".into(), DEF_STYLE()))
            .add_str("lush".into(), ContentStyle::new().green().italic())
            .add_fun(|| {
                env::current_dir().map_or("ERR".to_string(), |f| f.to_string_lossy().to_string())
            }, ContentStyle::new().cyan().underlined())
            .add_line(P::Str(" ".into(), DEF_STYLE()))
            .add_str("-> ".into(), ContentStyle::new().yellow().bold())
    }
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
                    .join(&x.1.to_string())
            })
            .collect();

        write!(f, "{}", lines.join("\n"))
    }
}
