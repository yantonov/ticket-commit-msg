use std::env;
use std::path::{Path, PathBuf};
use crate::{process, GIT_CONFIG_PREFIX_PARAM};

pub const TICKET_PREFIX_ENV_VAR: &str = "TICKET_PREFIX";

pub struct Environment {
    executable: String,
    commit_msg_tmp_file: Option<String>,
    prefix: Option<String>,
}

impl Environment {
    pub fn commit_msg_file(&self) -> Result<PathBuf, String> {
        if self.commit_msg_tmp_file == None {
            Err("Commit message temporary file should be passed as first argument".to_string())
        } else {
            Ok(Path::new(&self.commit_msg_tmp_file.clone().unwrap())
                .canonicalize()
                .unwrap())
        }
    }

    pub fn executable_name(&self) -> String {
        Path::new(&self.executable.clone())
            .file_name()
            .expect("Fail to get executable file name")
            .to_str()
            .expect("Fail to convert executable file name to string")
            .to_string()
    }
    
    pub fn prefix(&self) -> Option<String> {
        self.prefix.clone()
    }

    pub fn show_usage(&self) -> bool {
        self.commit_msg_tmp_file == None
    }
}

pub fn system_environment() -> Result<Environment, String> {
    let args: Vec<String> = env::args().collect();
    let ticket_prefix_from_config = match process::exec(
        "git",
        &["config", GIT_CONFIG_PREFIX_PARAM]) {
        Ok(prefix) => {
            if prefix.trim().len() > 0 {
                Some(prefix)
            }
            else {
                None
            }
        },
        Err(_) => None
    };
    let ticket_prefix_from_env = match env::var(TICKET_PREFIX_ENV_VAR) {
        Ok(prefix) => Some(prefix),
        Err(_) => None
    };
    
    Ok(Environment {
        executable: args.get(0)
            .expect("executable is not defined")
            .to_string(),
        commit_msg_tmp_file: args.get(1).cloned(),
        prefix: ticket_prefix_from_config.or(ticket_prefix_from_env),
    })
}