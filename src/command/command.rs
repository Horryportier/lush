use std::process::{Child, Stdio};

use crate::{colors::colors::ERROR_STYLE, error::error::LushError};

#[derive(Debug, Clone)]
pub enum InputType {
    Pipe,
    Cmd(Command),
    Break,
}

pub enum SpacialCommand {
    Exit,
    CD(Vec<String>),
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Input {
    pub commands: Option<Vec<InputType>>,
}

impl Input {
    pub fn parse(input: &str) -> Input {
        let err_style = ERROR_STYLE();

        let parts = input
            .split_whitespace()
            .into_iter()
            .map(|x| {
                if x.ends_with("|") {
                    vec![x.split("|").collect::<Vec<&str>>().first().unwrap(), "|"]
                } else if x.ends_with(";") {
                    vec![x.split(";").collect::<Vec<&str>>().first().unwrap(), ";"]
                } else {
                    vec![x]
                }
            })
            .collect::<Vec<Vec<&str>>>()
            .concat();
        let parts = parts
            .iter()
            .filter(|x| x != &&"")
            .copied()
            .collect::<Vec<&str>>();

        if parts.clone().len() == 0 {
            return Input { commands: None };
        }
        let mut cmds: Vec<InputType> = Vec::new();
        let mut cmd: Vec<String> = Vec::new();
        for (i, part) in parts.into_iter().enumerate() {
            match part.trim() {
                "|" => {
                    if i == 0 {
                        eprintln!(
                            "{}",
                            err_style.apply("can not use pipe \"|\"  as first argument")
                        );
                        return Input { commands: None };
                    }
                    if !cmd.is_empty() {
                        let mut cmd = cmd.into_iter();
                        let c = cmd.next().unwrap();
                        let a = cmd.collect();
                        cmds.push(InputType::Cmd(Command { cmd: c, args: a }))
                    }
                    cmd = Vec::new();
                    cmds.push(InputType::Pipe);
                }
                ";" => {
                    if i == 0 {
                        eprintln!(
                            "{}",
                            err_style.apply("can not use break \";\" as first argument")
                        );
                        return Input { commands: None };
                    }
                    if !cmd.is_empty() {
                        let mut cmd = cmd.into_iter();
                        let c = cmd.next().unwrap();
                        let a = cmd.collect();
                        cmds.push(InputType::Cmd(Command { cmd: c, args: a }))
                    }
                    cmd = Vec::new();
                    cmds.push(InputType::Break);
                }
                _ => cmd.push(part.into()),
            }
        }
        if !cmd.is_empty() {
            let mut cmd = cmd.into_iter();
            let c = cmd.next().unwrap();
            let a = cmd.collect();
            cmds.push(InputType::Cmd(Command { cmd: c, args: a }))
        }
        return Input {
            commands: Some(cmds),
        };
    }

    pub fn eval(self) -> Result<Option<SpacialCommand>, LushError> {
        if let Some(cmds) = self.commands {
            let mut cmds = cmds.iter();
            let mut prev_command: Option<Child> = None;
            while let Some(cmd_type) = cmds.next() {
                match cmd_type {
                    InputType::Pipe => continue,
                    InputType::Break => continue,
                    InputType::Cmd(command) => match command.cmd.as_str() {
                        "exit" => return Ok(Some(SpacialCommand::Exit)),
                        "cd" => return Ok(Some(SpacialCommand::CD(command.clone().args))),
                        _ => {
                            let stdin =
                                prev_command.map_or(Stdio::inherit(), |mut output: Child| {
                                    Stdio::from(match output.stdout.take() {
                                        Some(s) => Stdio::from(s),
                                        None => Stdio::inherit(),
                                    })
                                });

                            let stdout = match cmds.clone().peekable().peek() {
                                None => Stdio::inherit(),
                                Some(cmd_type) => match cmd_type {
                                    InputType::Cmd(..) => {
                                        return Err(LushError::LushErr(
                                            "comand one fater another".into(),
                                        ))
                                    }
                                    InputType::Break => Stdio::inherit(),
                                    InputType::Pipe => Stdio::piped(),
                                },
                            };
                            let output = std::process::Command::new(command.clone().cmd)
                                .args(command.clone().args)
                                .stdin(stdin)
                                .stdout(stdout)
                                .spawn();
                            match output {
                                Ok(out) => {
                                    prev_command = Some(out);
                                }
                                Err(e) => {
                                    prev_command = None;
                                    eprintln!("{e}")
                                }
                            }
                        }
                    },
                }
            }
            if let Some(mut final_command) = prev_command {
                let _ = final_command.wait();
            }
        }

        Ok(None)
    }
}
