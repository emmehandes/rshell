use std::io::{self, Write};
use std::process::Command;
use std::collections::HashMap;

fn main() {
    //TODO: record used commands and their arguments
    //let mut commands: HashMap<&str, Vec<&str> > = HashMap::new();
    //
    loop {
        // TODO: Default style to configure
        print!("$ ");
        let reader = io::stdin();
        let mut writer = io::stdout();

        // TODO: create a logging mechanism for the application
        writer.flush().expect("Could not flush");

        // Read line and run command
        let mut input: String = String::new();
        reader.read_line(&mut input).ok().expect("Could Not read");

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();

        let mut child = Command::new(command)
            .args(parts)
            .spawn()
            .expect("Failed to execute child process");

        child.wait().expect("Failed to wait on child process");
    }
}
