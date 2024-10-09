use std::env;
use std::io;
use std::path::Path;
use sysinfo::{Pid, System};

fn main() {
    let system = System::new_all();
    loop {
        let command = read_command_input();
        let commands_split = parse_commands_input(command);

        for command in commands_split {
            command.execute(&system);
        }
    }
}

fn read_command_input() -> String {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read in command");
    return command;
}

fn parse_commands_input(command: String) -> Vec<Command> {
    return command
        .trim()
        .split("|")
        .map(|command| return Command::from_string(command))
        .collect();
}

#[derive(Debug)]
enum CommandType {
    Cd(String),
    Kill(usize),
    Echo(String),
    Ps,
    Pwd,
    Quit,
    Unknown(String),
}

struct Command {
    _cmd: CommandType,
}
impl Command {
    pub fn from_string(command: &str) -> Self {
        let mut parts = command.split_whitespace();

        let cmd = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        let command_type = match cmd {
            "cd" => {
                if let Some(path) = args.get(0) {
                    CommandType::Cd(path.to_string())
                } else {
                    CommandType::Unknown(format!("Cannot parse command {command}"))
                }
            }
            "kill" => {
                if let Some(pid_str) = args.get(0) {
                    if let Ok(pid) = pid_str.parse::<usize>() {
                        CommandType::Kill(pid)
                    } else {
                        CommandType::Unknown(format!("Incorrect argument {:?}", args.join("")))
                    }
                } else {
                    CommandType::Unknown(format!("No PID passed"))
                }
            }
            "echo" => {
                let message = args.join(" ");
                CommandType::Echo(message)
            }
            "ps" => CommandType::Ps,
            "pwd" => CommandType::Pwd,
            "\\quit" => CommandType::Quit,
            _ => CommandType::Unknown("Unknown command".to_string()),
        };

        Command { _cmd: command_type }
    }

    pub fn execute(&self, system: &System) {
        match &self._cmd {
            CommandType::Cd(path) => {
                let new_path = Path::new(&path);
                env::set_current_dir(&new_path).expect("Failed to change directory");
            }
            CommandType::Kill(proccess_id) => {
                if let Some(process) = system.process(Pid::from(*proccess_id)) {
                    process.kill();
                } else {
                    eprintln!("Cannot find process PID {proccess_id}");
                }
            }
            CommandType::Echo(args) => {
                println!("{}", args);
            }
            CommandType::Ps => {
                println!("Id        t/ms         Name");
                for (pid, process) in system.processes() {
                    let mut result_pid_string = pid.to_string();
                    let mut process_runtime_string = process.run_time().to_string();

                    result_pid_string += &" ".repeat(10 - result_pid_string.len());
                    process_runtime_string += &" ".repeat(12 - process_runtime_string.len());

                    println!(
                        "{}{}{:?}",
                        result_pid_string,
                        process_runtime_string,
                        process.name()
                    );
                }
            }
            CommandType::Pwd => {
                let current_path = env::current_dir().expect("Cannot get current path");
                println!("{:?}", current_path.display());
            }
            CommandType::Unknown(message) => eprintln!("{message}"),
            CommandType::Quit => std::process::exit(0),
        }
    }
}
