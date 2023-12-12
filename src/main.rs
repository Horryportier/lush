use command::command::{Input, SpacialCommand};
use crossterm::style::{ContentStyle, Stylize};
use error::error::LushError;
use prompt::prompt::{Prompt, P};
use std::{
    io::stdin,
    vec,
};
mod colors;
mod command;
mod error;
mod prompt;

fn get_input(string: &mut String) {
    stdin()
        .read_line(string)
        .ok()
        .expect("Failed to read user input");
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

    let no_style = ContentStyle::new();
    let print_err = |e: LushError| eprintln!("{}", e.to_string().red());
    let prompt = Prompt {
        lines: vec![
            (
                vec![
                    P::Str("lush".into(), lush_style),
                    P::Env("PWD".into(), curr_dir_style),
                    P::Str(
                        "🦀".into(),
                        curr_dir_style.attribute(crossterm::style::Attribute::NoUnderline),
                    ),
                ],
                P::Str("::".into(), no_style),
            ),
            (
                vec![P::Str("=> ".into(), arrow_style)],
                P::Str(" ".into(), no_style),
            ),
        ],
    };
    loop {
        let mut user_inuput = String::new();
        match Prompt::default().render() {
            Ok(..) => (),
            Err(..) => eprintln!("can't render prompt"),
        }
        get_input(&mut user_inuput);
        if let Ok(spacial_command) = Input::parse(user_inuput.as_str()).eval() { 
            if let Some(cmd) =  spacial_command {
                match cmd {
                    SpacialCommand::Exit => return,
                    SpacialCommand::CD(_) => todo!(),
                }
            }
        } 
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
