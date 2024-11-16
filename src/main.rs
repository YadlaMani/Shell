use std::env;
use std::io::{stdout, stdin, Write};
use std::process::{Command, Stdio};
use std::path::Path;

fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.split_whitespace();
            let command_name = parts.next().unwrap_or("");
            let args = parts;

            match command_name {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                },
                "exit" => return,
                _ => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(), |output: std::process::Child| {
                            Stdio::from(output.stdout.unwrap())
                        });
                    
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command_name)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(child) => {
                            previous_command = Some(child);
                        },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
