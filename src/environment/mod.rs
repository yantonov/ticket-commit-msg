use std::env;
use std::path::{Path, PathBuf};

pub struct Environment {
    commit_msg_tmp_file: PathBuf,
}

impl Environment {
    pub fn commit_msg_file(&self) -> &PathBuf {
        &self.commit_msg_tmp_file
    }
}

struct SystemEnvironment {}

impl SystemEnvironment {
    pub fn commit_msg_tmp_file(&self) -> Result<PathBuf, String> {
        let args: Vec<String> = env::args().collect();
        let temporary_file = args.get(1)
            .map(|x| x.clone());
        match temporary_file {
            None => Err("Commit message temporary file should be passed as first argument".to_string()),
            Some(value) => Ok(Path::new(&value).canonicalize().unwrap())
        }
    }
}

pub fn system_environment() -> Result<Environment, String> {
    let sys_env = SystemEnvironment {};
    Ok(Environment {
        commit_msg_tmp_file: sys_env.commit_msg_tmp_file()?
    })
}