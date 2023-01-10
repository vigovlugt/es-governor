use std::process::Command;

pub fn run_command(command: &str) -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("sh").arg("-c").arg(command).status()
}
