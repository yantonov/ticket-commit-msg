use environment::Environment;

mod environment;
mod file;
mod ticket_number;
mod process;
mod patch_commit_msg;

const GIT_CONFIG_PREFIX_PARAM: &str = "custom.ticketnumberprefix";

fn usage(env: &Environment) -> Result<(), String> {
    println!("ticket-commit-msg");
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: {} COMMIT_MESSAGE_FILE", env.executable_name());
    println!();
    println!("To set prefix for the ticket number:");
    println!("git config {} PREFIX_VALUE", GIT_CONFIG_PREFIX_PARAM);
    println!();
    println!("or use {} env var", environment::TICKET_PREFIX_ENV_VAR);
    Ok(())
}

fn adjust_commit_message(env: &Environment) -> Result<(), String> {
    let commit_msg_file = env.commit_msg_file()?;
    let commit_msg = file::read_file(&commit_msg_file)?;
    let branch = process::exec(
        "git",
        &["rev-parse", "--abbrev-ref", "HEAD"])
        .map_err(|err| format!("cannot detect current branch: [details: {}]", err))?;
    let ticket_number = ticket_number::ticket_number(&branch);
    if ticket_number.is_some() {
        let updated_commit_msg = patch_commit_msg::patch_commit_msg(
            &commit_msg,
            &ticket_number,
            &env.prefix());
        file::write_file(&commit_msg_file, &updated_commit_msg)?;
    }
    Ok(())
}

fn entry_point() -> Result<(), String> {
    let env = environment::system_environment()?;
    if env.show_usage() {
        usage(&env)
    } else {
        adjust_commit_message(&env)
    }
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
