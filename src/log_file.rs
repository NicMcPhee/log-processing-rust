use anyhow::Result;
use itertools::Itertools;

use crate::failed_login::FailedLogin;

pub struct LogFile {}

impl LogFile {
    pub fn process_log_files(log_files: &[String]) -> Result<Vec<FailedLogin>> {
        log_files
            .iter()
            .map(|filename| Self::process_log_file(filename))
            .flatten_ok()
            .collect::<Result<_>>()
    }

    fn process_log_file(file: &str) -> Result<Vec<FailedLogin>> {
        todo!()
    }
}
