use std::env;
use std::path::{Path, PathBuf};

pub struct Environment {
    executable: String,
    commit_msg_tmp_file: Option<String>,
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

    pub fn show_usage(&self) -> bool {
        self.commit_msg_tmp_file == None
    }
}

pub fn system_environment() -> Result<Environment, String> {
    let args: Vec<String> = env::args().collect();
    Ok(Environment {
        executable: args.get(0)
            .expect("executable is not defined")
            .to_string(),
        commit_msg_tmp_file: args.get(1).cloned(),
    })
}