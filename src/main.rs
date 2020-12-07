mod environment;
mod file;
mod ticket_number;
mod process;
mod patch_commit_msg;

fn entry_point() -> Result<(), String> {
    let env = environment::system_environment()?;
    let commit_msg_file = env.commit_msg_file();
    let commit_msg = file::read_file(commit_msg_file)?;
    let branch = process::exec(
        "git",
        &vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .map_err(|err| format!("cannot detect current branch: [details: {}]", err).to_string())?;
    let ticket_number = ticket_number::ticket_number(&branch);
    let ticket_prefix = match process::exec(
        "git",
        &vec!["config", "ticket.number.prefix"]) {
        Ok(prefix) => Some(prefix.trim().to_string()),
        Err(_) => None
    };
    let updated_commit_msg = patch_commit_msg::patch_commit_msg(
        &commit_msg,
        &ticket_number,
        &ticket_prefix);
    file::write_file(commit_msg_file, &updated_commit_msg)?;
    Ok(())
}

fn main() {
    match entry_point() {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    }
}
