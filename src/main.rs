use command::command::Input;
use crossterm::style::{ContentStyle, Stylize};
use prompt::prompt::{Prompt, P};
use std::{
    env,
    io::{self, stdin},
    path::Path,
    process::{Command, ExitStatus},
    vec,
};
use thiserror::Error;
mod prompt;
mod colors;
mod command;

#[derive(Debug, Error)]
pub enum LushError {
    #[error("Io error {0}")]
    IoErr(#[from] io::Error),
    #[error("Lush error {0}")]
    #[allow(dead_code)]
    LushErr(String),
}

fn get_input(string: &mut String) {
    stdin()
        .read_line(string)
        .ok()
        .expect("Failed to read user input");
}


fn run_cmd(cmd: &str, args: Vec<&str>) -> Result<ExitStatus, LushError> {
    let mut child = Command::new(cmd.trim()).args(args).spawn()?;
    Ok(child.wait()?)
}

fn main() {
    let lush_style = ContentStyle::new()
        .with(crossterm::style::Color::Green)
        .attribute(crossterm::style::Attribute::Italic);
    let curr_dir_style = ContentStyle::new()
        .with(crossterm::style::Color::Cyan)
        .attribute(crossterm::style::Attribute::Underdotted);
    let arrow_style = ContentStyle::new()
        .with(crossterm::style::Color::Red)
        .attribute(crossterm::style::Attribute::Bold);

    let no_style =  ContentStyle::new();
    let print_err = |e: LushError| eprintln!("{}", e.to_string().red());
    let prompt = Prompt {
        lines: vec![
            (
                vec![
                    P::Str("lush".into(), lush_style),
                    P::Env("PWD".into(), curr_dir_style),
                    P::Str("🦀".into(), curr_dir_style.attribute(crossterm::style::Attribute::NoUnderline)),
                ],
                P::Str("::".into(), no_style),
            ),
            (vec![P::Str("=> ".into(), arrow_style)], P::Str(" ".into(), no_style)),
        ],
    };
    loop {
        let mut user_inuput = String::new();
        match Prompt::default().render() {
            Ok(..) => (),
            Err(..) => eprintln!("can't render prompt"),
        }
        get_input(&mut user_inuput);
        println!("{:#?}", Input::parse(user_inuput.as_str()));
        //let mut parts = user_inuput.trim().split_whitespace();
        //let cmd = match parts.next() {
        //    Some(s) => s,
        //    None => continue,
        //};
        //let args = parts;
        //match cmd {
        //    "cd" => {
        //        let new_dir = args.clone().peekable().peek().map_or("/", |x| *x);
        //        let root = Path::new(new_dir);
        //        if let Err(e) = env::set_current_dir(&root) {
        //            eprintln!("{e}");
        //        };
        //    }
        //    "#" => {
        //        print!(
        //            "# {}",
        //            args.clone()
        //                .collect::<Vec<&str>>()
        //                .join(" ")
        //                .cyan()
        //                .italic()
        //        )
        //    }
        //    "exit" => break,
        //    _ => {
        //        let _exit_status = run_cmd(cmd, args.collect()).map_err(print_err);
        //        ()
        //    }
        //}
    }
}
