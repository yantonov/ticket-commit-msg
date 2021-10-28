use std::process::Command;

pub fn exec(command: &str, args: &[&str]) -> Result<String, String> {
    Command::new(command)
        .args(args)
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap())
        .map_err(|err| format!("failed to execute process: {}", err))
}