use environment::{Command, Environment};

mod environment;
mod file;
mod patch_commit_msg;
mod process;
mod ticket_number;

const GIT_CONFIG_PREFIX_PARAM: &str = "custom.ticketnumberprefix";

fn usage(env: &Environment) -> Result<(), String> {
    println!("ticket-commit-msg");
    // The commit comes from build.rs, so the line names the exact source the
    // binary was built from without the version number alone having to be enough.
    println!(
        "version: {} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );
    println!();
    println!("Usage: {} COMMIT_MESSAGE_FILE", env.executable_name());
    println!(
        "       {} {} BRANCH",
        env.executable_name(),
        environment::VALIDATE_FLAG
    );
    println!();
    println!(
        "{} checks that the branch name contains a ticket",
        environment::VALIDATE_FLAG
    );
    println!("number and exits with 0 when it does, 1 otherwise.");
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
    let branch = process::exec("git", &["rev-parse", "--abbrev-ref", "HEAD"])
        .map_err(|err| format!("cannot detect current branch: [details: {}]", err))?;
    let ticket_number = ticket_number::ticket_number(&branch);
    if ticket_number.is_some() {
        let updated_commit_msg =
            patch_commit_msg::patch_commit_msg(&commit_msg, &ticket_number, &env.prefix());
        file::write_file(&commit_msg_file, &updated_commit_msg)?;
    }
    Ok(())
}

fn validate_branch(env: &Environment, branch: &Option<String>) -> Result<(), String> {
    let Some(branch) = branch else {
        return Err(format!(
            "branch name should be passed after {}",
            environment::VALIDATE_FLAG
        ));
    };
    let branch = branch.trim();
    println!("branch:  {}", branch);
    match ticket_number::ticket_number(branch) {
        Some(ticket) => {
            println!("match:   yes");
            println!("ticket:  {}", ticket);
            println!(
                "line:    {}",
                patch_commit_msg::ticket_line(&ticket, &env.prefix())
            );
            Ok(())
        }
        None => {
            println!("match:   no");
            Err("no ticket number found in branch name".to_string())
        }
    }
}

fn entry_point() -> Result<(), String> {
    let env = environment::system_environment()?;
    match env.command() {
        Command::ShowUsage => usage(&env),
        Command::ValidateBranch(branch) => validate_branch(&env, branch),
        Command::PatchCommitMsg => adjust_commit_message(&env),
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
