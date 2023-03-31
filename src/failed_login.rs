use std::iter::empty;

use anyhow::Result;
use itertools::Itertools;

pub struct FailedLogin {}

impl FailedLogin {
    pub fn process_log_files(log_files: &[String]) -> Result<Vec<Self>> {
        log_files
            .iter()
            .map(|filename| Self::process_log_file(filename))
            .flatten_ok()
            .collect::<Result<_>>()
    }

    fn process_log_file(file: &str) -> Result<Vec<Self>> {
        todo!()
    }
}
