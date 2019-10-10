use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        // Default style to configure
        print!("$ ");
        let reader = io::stdin();
        let mut writer = io::stdout();

        writer.flush().expect("Could not flush");

        // Read line and run command
        let mut input: String = String::new();
        reader.read_line(&mut input).ok().expect("Could Not read");

        let mut child = Command::new(input.trim())
            .spawn()
            .expect("Failed to execute child process");

        child.wait().expect("Failed to wait on child process");
    }
}
