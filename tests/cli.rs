use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn set_context() -> Result<(), Box<dyn std::error::Error>> {
    let kubeconfig: String = format!("{}/.kube/config", dirs::home_dir().unwrap().display());
    let dest: String = format!(
        "{}/.kube/kubesess/cache",
        dirs::home_dir().unwrap().display()
    );

    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("-v docker-desktop").arg("context").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert_eq!(
        output_string,
        format!("{}/docker-desktop:{}", dest, kubeconfig)
    );

    Ok(())
}

#[test]
fn set_default_context() -> Result<(), Box<dyn std::error::Error>> {
    let kubeconfig: String = format!("{}/.kube/config", dirs::home_dir().unwrap().display());
    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v docker-desktop").arg("default-context").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert_eq!(
        output_string,
        format!("Switched to context \"docker-desktop\".\n{}", kubeconfig)
    );

    Ok(())
}

#[test]
fn set_namespace() -> Result<(), Box<dyn std::error::Error>> {
    let kubeconfig: String = format!("{}/.kube/config", dirs::home_dir().unwrap().display());
    let dest: String = format!(
        "{}/.kube/kubesess/cache",
        dirs::home_dir().unwrap().display()
    );


    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v default").arg("namespace").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert_eq!(
        output_string,
        format!("{}/docker-desktop:{}", dest, kubeconfig)
    );

    Ok(())
}

#[test]
fn set_default_namespace() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v default").arg("default-namespace").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert_eq!(
        output_string,
        format!("Context \"docker-desktop\" modified.")
    );

    Ok(())
}

