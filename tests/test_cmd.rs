use std::path::Path;

use assert_cmd::Command;
use assert_fs::{
    fixture::{FileWriteFile, FixtureError, PathChild},
    TempDir,
};

pub fn setup_tinycross() -> Result<TempDir, FixtureError> {
    let temp = TempDir::new()?;
    let input_file = temp.child("tinycross.png");
    input_file.write_file(&Path::new(env!("CARGO_MANIFEST_DIR")).join("images/tinycross.png"))?;
    Ok(temp)
}

#[test]
fn test_fails_without_input() {
    Command::cargo_bin("ign").unwrap().assert().failure();
}

#[test]
fn test_same_dir() {
    let temp = setup_tinycross().unwrap();
    Command::cargo_bin("ign")
        .unwrap()
        .current_dir(temp.path())
        .arg("tinycross.png")
        .arg("tinycross-nord.png")
        .assert()
        .success();
}

#[test]
fn test_same_dir_relative() {
    let temp = setup_tinycross().unwrap();
    Command::cargo_bin("ign")
        .unwrap()
        .current_dir(temp.path())
        .arg("./tinycross.png")
        .arg("./tinycross-nord.png")
        .assert()
        .success();
}
#[test]
fn test_same_dir_absolute() {
    let temp = setup_tinycross().unwrap();
    Command::cargo_bin("ign")
        .unwrap()
        .current_dir(temp.path())
        .arg(temp.path().join("tinycross.png").canonicalize().unwrap())
        .arg(
            temp.path()
                .join("tinycross-nord.png")
                .canonicalize()
                .unwrap(),
        )
        .assert()
        .success();
}

#[test]
fn test_dir() {
    let temp = setup_tinycross().unwrap();
    let out = TempDir::new().unwrap();
    Command::cargo_bin("ign")
        .unwrap()
        .arg(temp.path())
        .arg(out.path())
        .assert()
        .success();
}
