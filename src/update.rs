use std::error::Error;
use std::path::Path;
use std::process::Command;

pub struct UpdateOption<'a> {
    snapshot_key: &'a str,
    snapshot_dest_path: &'a Path,
}

pub fn run(command: &Vec<&str>, option: &UpdateOption) -> Result<(), Box<dyn Error>> {
    let (command_vec, arguments) = command.split_at(1);
    if let Some(command) = command_vec.first() {
        let output = Command::new(command).args(arguments).output()?;
    }
    Ok(())
}

#[test]
fn run_test() {
    assert!(run(
        &vec!["git", "exec"],
        &UpdateOption {
            snapshot_key: "ls-al",
            snapshot_dest_path: Path::new("./main.rs")
        }
    )
    .is_ok())
}
#[test]
fn split_at_test() {
    let v = vec!["git", "exec", "foobar"];
    let (left, right) = v.split_at(1);
    assert_eq!(left, ["git"]);
    assert_eq!(right, ["exec", "foobar"]);
}
