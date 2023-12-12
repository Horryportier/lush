use crate::colors::colors::ERROR_STYLE;

#[derive(Debug, Clone)]
pub enum InputType {
    Pipe,
    Cmd(Command),
    Break,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Command {
    cmd: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct Input {
    comamnds: Option<Vec<InputType>>,
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

        println!("{:?}", parts);
        if parts.clone().len() == 0 {
            return Input { comamnds: None };
        }
        let mut cmds: Vec<InputType> = Vec::new();
        let mut cmd: Vec<String> = Vec::new();
        for (i, part) in parts.into_iter().enumerate() {
            match part.trim() {
                "|" => {
                    if i == 0 {
                        eprintln!("{}", err_style.apply("can not use pipe \"|\"  as first argument"));
                        return Input { comamnds: None };
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
                        eprintln!("{}", err_style.apply("can not use break \";\" as first argument"));
                        return Input { comamnds: None };
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
            comamnds: Some(cmds),
        };
    }
}
