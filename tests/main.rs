use std::{
    io,
    process::{Command, Stdio},
};

// use assert_cmd::cargo::CargoError;
// use assert_cmd::{Command, pkg_name};

// This happens when piping into less and quitting before all output in consumed. This is perfectly
// acceptable behaviour that we should exit cleanly on
#[test]
pub fn exits_successfully_on_closed_pipe() -> Result<(), io::Error> {
    // thisone we have to do manually bc of the pipes
    #[allow(clippy::unwrap_used)]
    let mut cmd_child = Command::new("cargo")
        .arg("r")
        .arg("-q")
        .arg("--")
        .arg("objects.inv")
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
