use std::{
    fs::{self, File},
    io::{Read, BufReader, BufRead},
    ops::Not,
};

use anyhow::{bail, Context, Error, Result};
use flate2::read::GzDecoder;
use itertools::{Either, Itertools};
use tar::{Archive, Entry};

use crate::failed_login::FailedLogin;

pub struct LogFile {}

impl LogFile {
    pub fn process_tar_balls(tar_ball_names: &[String]) -> Result<Vec<FailedLogin>> {
        tar_ball_names
            .iter()
            .map(|tar_ball_name| Self::process_tar_ball(tar_ball_name))
            .flatten_ok()
            .collect::<Result<_>>()
    }

    fn process_tar_ball(tar_ball_name: &str) -> Result<Vec<FailedLogin>> {
        let tar_gz = File::open(tar_ball_name)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        let (successes, failures): (Vec<_>, Vec<_>) = archive
            .entries()
            .with_context(|| format!("Failed to get the entry list from '{tar_ball_name}'"))?
            .into_iter()
            .partition_map(|r| match r {
                Ok(v) => Either::Left(v),
                Err(v) => Either::Right(v),
            });

        if failures.is_empty().not() {
            bail!("These entries in {tar_ball_name} couldn't be extracted: {failures:?}");
        }

        Ok(successes.into_iter().map(|e| Self::process_log_file(e)).concat())
    }

    fn process_log_file(entry: Entry<GzDecoder<File>>) -> Vec<FailedLogin> {
        let result = BufReader::new(entry)
            .lines()
            .map_ok(|line| line);
        // let mut buffer = String::new();
        // let _ = entry.read_to_string(&mut buffer);
        //     buffer
        // });
        todo!()
    }
}
