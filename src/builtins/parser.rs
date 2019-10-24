/// TODO: ADD DOCUMENTATION
/// TODO: Pass messages regarding errors and success
/// TODO: save in a map the commands and their arguments
///
use std::env;
use std::fmt;
use std::path::Path;
use std::process::Command;

struct Request<'a> {
    name: &'a str,
    args: Vec<&'a str>,
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}\n args:{:?}", self.name, self.args)
    }
}

//-----------------------------------------------------------------------------
// Parse command
pub fn parse(mut command: std::str::SplitWhitespace) {
    let request = Request {
            name: command.next().unwrap(),
            args: command.collect() };

    match request.name {
        "exit" => { exit(0) },
        "cd" => { if request.args.len() >= 1 { cd(request.args[1]); } }
        &_ => { execute(request); }
    }
}

//-----------------------------------------------------------------------------
// exit program
fn exit(code: i32) {
    std::process::exit(code);
}

//-----------------------------------------------------------------------------
// cd change directory function
fn cd(target: &str) {
    let current_dir: String = env::current_dir().unwrap().to_str().unwrap().to_owned();
    let new_dir = current_dir + "/" + target;
    if let Err(e) = env::set_current_dir(Path::new(&new_dir)) {
        eprintln!("{}", e);
    }
}

//-----------------------------------------------------------------------------
// execute request
fn execute<'a>(request: Request<'a>) {
    println!("{}", request);
    let child = Command::new(request.name)
                    .args(request.args)
                    .spawn();
    match child {
        Ok(mut child) => { child.wait().expect("Failed to execute child process").code(); }
        Err(e) => { eprintln!("{}", e); }
    }
}

