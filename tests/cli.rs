use assert_cmd::prelude::*;
use std::{env, process::Command};

#[test]
fn set_context() -> Result<(), Box<dyn std::error::Error>> {
    let expected: String = format!(
        "{}/.kube/kubesess/cache/docker-desktop:{}",
        dirs::home_dir().unwrap().display(),
        env::var("KUBECONFIG").unwrap()
    );

    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("-v docker-desktop").arg("context").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();
    assert_eq!(output_string, expected);

    Ok(())
}

#[test]
fn set_default_context() -> Result<(), Box<dyn std::error::Error>> {
    let expected: String = env::var("KUBECONFIG").unwrap();

    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v docker-desktop").arg("default-context").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert_eq!(output_string, expected);

    Ok(())
}

#[test]
fn set_namespace() -> Result<(), Box<dyn std::error::Error>> {
    let expected: String = format!(
        "{}/.kube/kubesess/cache/docker-desktop:{}",
        dirs::home_dir().unwrap().display(),
        env::var("KUBECONFIG").unwrap()
    );

    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v default").arg("namespace").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert_eq!(output_string, expected);

    Ok(())
}

#[test]
fn set_default_namespace() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v default").arg("default-namespace").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert_eq!(output_string, format!(""));

    Ok(())
}
