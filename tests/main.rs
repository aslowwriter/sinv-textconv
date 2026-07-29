#![allow(clippy::unwrap_used)]
use pretty_assertions::assert_eq;
use std::{
    fs, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use assert_cmd::{Command as CargoCommand, pkg_name};

// This happens when piping into less and quitting before all output in consumed. This is perfectly
// acceptable behaviour that we should exit cleanly on
#[test]
pub fn exits_successfully_on_closed_pipe() -> Result<(), io::Error> {
    // thisone we have to do manually bc of the pipes
    // here we can allow unwraps bc if anything goes wrong we
    // just want to fail the test
    #[allow(clippy::unwrap_used)]
    let mut cmd_child = Command::new("cargo")
        .arg("r")
        .arg("-q")
        .arg("--")
        .arg("test_files/minimal.inv")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    #[allow(clippy::unwrap_used)]
    let head = Command::new("head")
        .arg("-n1")
        .stdin(Stdio::from(cmd_child.stdout.take().unwrap()))
        .output()
        .unwrap();

    assert!(head.status.success());
    assert!(cmd_child.wait()?.success());

    Ok(())
}

#[test]
pub fn correct_output() -> Result<(), io::Error> {
    let path = PathBuf::from("test_files/minimal.inv");
    let expected: String = fs::read_to_string("test_files/minimal.txt")?;

    let output = CargoCommand::cargo_bin(pkg_name!())
        .unwrap()
        .arg(path)
        .output()?;

    assert!(output.status.success());

    assert_eq!(expected, String::from_utf8(output.stdout).unwrap());

    Ok(())
}
