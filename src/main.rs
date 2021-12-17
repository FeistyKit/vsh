mod commands;
mod eval;
mod prompt;
mod running;

use prompt::Prompt;

const SHELL_CHAR: char = 'λ';

fn main() {
    Prompt::new(SHELL_CHAR).start_shell().unwrap();
}
