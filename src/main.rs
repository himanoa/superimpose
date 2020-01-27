use std::env::var;
use clap::{App, Arg};
fn main() {
    let matches = App::new("superimpose")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Superimpose is snapshot test helper for command execution result.")
        .arg(Arg::with_name("key").help("snapshot key").takes_value(true).required(true))
        .arg(
            Arg::with_name("command")
                .help("snapshot target command")
                .empty_values(false)
                .multiple(true),
        )
        .get_matches();
    let command: Vec<_> = matches.values_of("command").unwrap().collect();
    let snapshot_key: String = matches.values_of("key").unwrap().collect();

    let is_update_snapshot = match var("SUPERIMPOSE_UPDATE_SNAPSHOT") {
        Ok(_) => true,
        Err(_) => false
    };

    let snapshot_path = match var("SUPERIMPOSE_SNAPHSHOT_PATH") {
        Ok(p) => p,
        _ => String::from("./snapshots")
    };
}
