use colored::Colorize;
use prompt::prompt::{Prompt, P};
use std::{
    env,
    io::{self, stdin, Write},
    path::Path,
    process::{Command, ExitStatus}, vec,
};
use thiserror::Error;
mod prompt;

#[derive(Debug, Error)]
enum LushError {
    #[error("Io error {0}")]
    IoErr(#[from] io::Error),
    #[error("Lush error {0}")]
    LushErr(String),
}

fn get_input(string: &mut String) {
    stdin()
        .read_line(string)
        .ok()
        .expect("Failed to read user input");
}

fn render_prompt(prompt: &Prompt) -> Result<(), LushError> {
    print!("\n{}", prompt);
    Ok(io::stdout().flush()?)
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> Result<ExitStatus, LushError> {
    let mut child = Command::new(cmd.trim()).args(args).spawn()?;
    Ok(child.wait()?)
}

fn main() {
    let print_err = |e: LushError| eprintln!("{}", e.to_string().red());
    let prompt = Prompt{lines: vec![(vec![P::Env("PWD".into()), P::Str(" =>".into())], " ".into())] };
    loop {
        let mut user_inuput = String::new();
        let _ = render_prompt(&prompt).map_err(print_err);
        get_input(&mut user_inuput);

        let mut parts = user_inuput.trim().split_whitespace();
        let cmd = parts.next().unwrap();
        let args = parts;
        match cmd {
            "cd" => {
                let new_dir = args.clone().peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{e}");
                };
            }
            "#" => {
                print!(
                    "# {}",
                    args.clone()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .cyan()
                        .italic()
                )
            }
            "exit" => break,
            _ => {
                let _exit_status = run_cmd(cmd, args.collect()).map_err(print_err);
                ()
            }
        }
    }
}
