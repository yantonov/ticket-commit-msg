use crate::{GIT_CONFIG_PREFIX_PARAM, process};
use std::env;
use std::path::{Path, PathBuf};

pub const TICKET_PREFIX_ENV_VAR: &str = "TICKET_PREFIX";
pub const VALIDATE_FLAG: &str = "--validate";

pub enum Command {
    ShowUsage,
    // The branch is optional so that a missing argument can be reported as a
    // run time error instead of silently falling back to the usage screen.
    ValidateBranch(Option<String>),
    PatchCommitMsg,
}

pub struct Environment {
    executable: String,
    command: Command,
    commit_msg_tmp_file: Option<String>,
    prefix: Option<String>,
}

impl Environment {
    pub fn commit_msg_file(&self) -> Result<PathBuf, String> {
        if self.commit_msg_tmp_file.is_none() {
            Err("Commit message temporary file should be passed as first argument".to_string())
        } else {
            Ok(Path::new(self.commit_msg_tmp_file.as_deref().unwrap())
                .canonicalize()
                .map_err(|e| format!("Cannot resolve commit message file path: {}", e))?)
        }
    }

    pub fn executable_name(&self) -> String {
        Path::new(&self.executable)
            .file_name()
            .expect("Fail to get executable file name")
            .to_str()
            .expect("Fail to convert executable file name to string")
            .to_string()
    }

    pub fn prefix(&self) -> Option<String> {
        self.prefix.clone()
    }

    pub fn command(&self) -> &Command {
        &self.command
    }
}

pub fn system_environment() -> Result<Environment, String> {
    let args: Vec<String> = env::args().collect();
    let ticket_prefix_from_config = match process::exec("git", &["config", GIT_CONFIG_PREFIX_PARAM])
    {
        Ok(prefix) => {
            if !prefix.trim().is_empty() {
                Some(prefix)
            } else {
                None
            }
        }
        Err(_) => None,
    };
    let ticket_prefix_from_env = env::var(TICKET_PREFIX_ENV_VAR).ok();

    let arg1 = args.get(1);
    // git always passes an existing path as the first argument, so a flag that
    // happens to name a real file is still treated as a commit message file.
    let names_an_existing_file = arg1.is_some_and(|a| Path::new(a).exists());
    let command = match arg1.map(String::as_str) {
        None => Command::ShowUsage,
        Some(_) if names_an_existing_file => Command::PatchCommitMsg,
        Some("--help") | Some("-h") => Command::ShowUsage,
        Some(VALIDATE_FLAG) => Command::ValidateBranch(args.get(2).cloned()),
        Some(_) => Command::PatchCommitMsg,
    };

    Ok(Environment {
        executable: args.first().expect("executable is not defined").to_string(),
        command,
        commit_msg_tmp_file: arg1.cloned(),
        prefix: ticket_prefix_from_config.or(ticket_prefix_from_env),
    })
}
