use std::path::Path;

mod environment;
mod file;
mod ticket_number;

fn entry_point() -> Result<(), String> {
    let env = environment::system_environment()?;
    let commit_msg_file = env.commit_msg_file();
    let commit_msg = file::read_file(commit_msg_file)?;
    file::write_file(commit_msg_file, &commit_msg);
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
