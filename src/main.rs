use command::command::{Input, SpacialCommand};
use crossterm::style::Stylize;
use error::error::LushError;
use prompt::prompt::Prompt;
use std::{env, io::stdin, path::Path};
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
    let print_err = |e: LushError| eprintln!("{}", e.to_string().red());

    loop {
        let mut user_inuput = String::new();
        match Prompt::default().render() {
            Ok(..) => (),
            Err(..) => eprintln!("can't render prompt"),
        }
        get_input(&mut user_inuput);
        match Input::parse(user_inuput.as_str()).eval() {
            Ok(sc) => {
                if let Some(cmd) = sc {
                    match cmd {
                        SpacialCommand::Exit => return,
                        SpacialCommand::CD(args) => {
                            let new_dir = args
                                .into_iter()
                                .peekable()
                                .peek()
                                .map_or("/".to_string(), |x| x.clone());
                            let root = Path::new(&new_dir);
                            if let Err(err) = env::set_current_dir(root) {
                                print_err(LushError::IoErr(err))
                            }
                        }
                    }
                }
            }
            Err(e) => print_err(e),
        }
    }
}
