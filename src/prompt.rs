use std::env;
use std::fs::File;
use std::io;
use std::process;

use lazy_static::lazy_static;
use crate::eval::{CommandError, Internalcommand};
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Prompt {
    character: char,
}

impl Prompt {
    pub fn new(character: char) -> Self {
        Self { character }
    }

    pub fn start_shell(&mut self) -> io::Result<()> {
        let mut rl = Editor::<()>::new();
        let home_dir = env::var("HOME").unwrap(); // There should be a HOME dir so no need to worry about this unwrap

        if rl
            .load_history(&format!("{}/.vsh_history", home_dir))
            .is_err()
        {
            eprintln!("No previous history.");
            File::create(format!("{}/.vsh_history", home_dir)).expect("Can't create history File!");
        }

        loop {
            let current_dir = std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();
            let dir_prompt = format!("   {} ", current_dir);
            let shell_char = format!("{}", self.character);

            println!(
                "{}{}{}",
                "".truecolor(109, 152, 134),
                dir_prompt
                    .on_truecolor(109, 152, 134)
                    .truecolor(33, 33, 33)
                    .bold(),
                "".truecolor(109, 152, 134)
            );
            let readline = rl.readline(format!("{} ", shell_char.green()).as_str());

            match readline {
                Ok(x) => {
                    rl.add_history_entry(x.as_str());
                    if let Err(e) = Self::run(x) {
                        match e {
                            CommandError::Exit => {
                                rl.save_history(&format!("{}/.vsh_history", home_dir))
                                    .expect("Couldn't Save History");
                                process::exit(0);
                            }
                            _ => {} // TODO: What should happen if an error is returned?
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => println!(),
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
            rl.save_history(&format!("{}/.vsh_history", home_dir))
                .expect("Couldn't Save History");
        }
        Ok(())
    }

    pub fn run(x: String) -> Result<(), CommandError> {
        let mut last_return = Ok(());
        for com in x.split(";") {
            last_return = Self::run_linked_commands(com.into());
        }
        last_return
    }
    fn run_command(com: String) -> Result<(), CommandError> {
        Internalcommand::new(com.to_string()).eval()
    }
    fn run_linked_commands(commands: String) -> Result<(), CommandError> {
        for linked_com in commands.split("&&") {
            if let Err(e) = Self::run_command(linked_com.to_string()) {
                return Err(e);
            }
        }
        Ok(())
    }
}

// TODO: Stop relying so much on regexes when they're not really needed
lazy_static! {
    static ref SEMICOLON: fancy_regex::Regex = fancy_regex::Regex::new("(?<!\\\\)\\;$").unwrap();
    static ref QUOTE_START: fancy_regex::Regex = fancy_regex::Regex::new("^\"").unwrap();
    static ref QUOTE_END: fancy_regex::Regex = fancy_regex::Regex::new("(?<!\\\\)\\\"$").unwrap();
}



#[derive(Debug)]
pub enum CommandStructure {
    And {lhs: Box<CommandStructure>, rhs: Box<CommandStructure>},
    Or {lhs: Box<CommandStructure>, rhs: Box<CommandStructure>},
    Both {lhs: Box<CommandStructure>, rhs: Box<CommandStructure>},
    Raw {command: String, args: Vec<String>},
}

impl CommandStructure {

    fn new(i: Vec<String>) -> Result<CommandStructure, &'static str> {
        if i.is_empty() {
            return Ok(Self::Raw {
                command: "".into(),
                args: Vec::new(),
            });
        }
        if i[0] == "&&" || &i[0] == "||" {
            return Err("Incorrect syntax!");
        }

        let mut command = i[0].clone();
        let mut args = Vec::new();
        for idx in 1..i.len() {
            if SEMICOLON.is_match(&i[idx]) {
                return Ok(Self::Both {
                    lhs: Box::new(Self::Raw {
                        command:
                    })
                })
            }
        }

        todo!()
    }
}

// This "()" can be changed if we want better error handling
fn format_quotes(i: Vec<String>) -> Result<Vec<String>, ()> {
    let mut curr = None;
    let mut to_return = Vec::with_capacity(i.len());
    for word in i {
        if QUOTE_START.is_match(&word).unwrap() {
            if curr.is_none() {
                word.remove(0);
                curr = Some(word);
                continue;
            } else {
                return Err(());
            }
        }

        if curr.is_some() {
            if !QUOTE_END.is_match(&word).unwrap() {
                curr.unwrap().extend(word.chars());
            } else {
                let mut to_append = curr.take().unwrap();
                to_append.extend(word.chars());
                to_return.push(to_append);
            }
            continue;
        }
        to_return.push(word);
    }
    Ok(to_return)
}
