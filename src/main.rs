use std::io::{self, Write};
mod builtins;

fn main() {
    loop {
        //
        // TODO: Default style to configure
        print!("$ ");
        let reader = io::stdin();
        let mut writer = io::stdout();

        // TODO: create a logging mechanism for the application
        writer.flush().expect("Could not flush");

        // Read line
        let mut input: String = String::new();
        reader.read_line(&mut input).ok().expect("Could Not read");
        let line = input.trim().split_whitespace();

        // Parse line
        builtins::parser::parse(line);
    }
}
