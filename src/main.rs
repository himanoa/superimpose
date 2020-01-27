mod command_snapshot;
mod update;

use clap::{App, Arg};
use std::env::var;
use std::error::Error;
use std::path::Path;
use update::{run, UpdateOption};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("superimpose")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Superimpose is snapshot test helper for command execution result.")
        .arg(
            Arg::with_name("key")
                .help("snapshot key")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("command")
                .help("snapshot target command")
                .empty_values(false)
                .multiple(true),
        )
        .get_matches();
    let command: Vec<_> = matches.values_of("command").unwrap().collect();
    let snapshot_key: String = matches.values_of("key").unwrap().collect();
    let snapshot_path = match var("SUPERIMPOSE_SNAPHSHOT_PATH") {
        Ok(p) => p,
        _ => String::from("./snapshots"),
    };

    match var("SUPERIMPOSE_UPDATE_SNAPSHOT") {
        Ok(_) => {
            run(
                &command,
                &UpdateOption {
                    snapshot_key: snapshot_key.as_str(),
                    snapshot_dest_path: Path::new(snapshot_path.as_str()),
                },
            )?;
        }
        Err(_) => {}
    };

    Ok(())
}
