use serde_json::to_string_pretty;
use std::error::Error;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

use crate::command_snapshot::CommandSnapshot;

pub struct UpdateOption<'a> {
    pub snapshot_key: &'a str,
    pub snapshot_dest_path: &'a Path,
}

fn write_snapshot(
    snapshot: &CommandSnapshot,
    option: &UpdateOption,
) -> Result<usize, Box<dyn Error>> {
    let json = to_string_pretty(snapshot)?;
    create_dir_all(option.snapshot_dest_path)?;
    let mut buffer = BufWriter::new(
        OpenOptions::new().write(true).create(true).open(
            option
                .snapshot_dest_path
                .join(format!("{}.json", option.snapshot_key)),
        )?,
    );
    match buffer.write(json.as_bytes()) {
        Ok(u) => Ok(u),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn run(command: &Vec<&str>, option: &UpdateOption) -> Result<(), Box<dyn Error>> {
    let (command_vec, arguments) = command.split_at(1);
    if let Some(command) = command_vec.first() {
        let output = Command::new(command).args(arguments).output()?;
        let snapshot = CommandSnapshot {
            status: output.status.code().unwrap(),
            stdout: String::from_utf8(output.stdout)?,
            stderr: String::from_utf8(output.stderr)?,
        };
        write_snapshot(&snapshot, option)?;
    }
    Ok(())
}

