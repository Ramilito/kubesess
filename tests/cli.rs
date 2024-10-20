use assert_cmd::prelude::*;
use serial_test::serial;
use std::{env, fs::File, io::Write, process::Command};
use tempfile::TempDir;

fn setup_environment() -> (TempDir, String, String) {
    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Set the HOME environment variable
    env::set_var("HOME", temp_dir.path());

    // Create a file named `config` inside the temporary directory
    let kubeconfig_path = temp_dir.path().join("config");
    let mut kubeconfig_file =
        File::create(&kubeconfig_path).expect("Failed to create kubeconfig file");

    // Write the Kubernetes configuration content to the file
    let kubeconfig_content = r#"
apiVersion: v1
clusters:
- cluster:
    server: https://kubernetes.docker.internal:6443
  name: docker-desktop
contexts:
- context:
    cluster: docker-desktop
    namespace: default
    user: docker-desktop
  name: docker-desktop
current-context: docker-desktop
kind: Config
preferences: {}
users:
- name: docker-desktop
"#;
    kubeconfig_file
        .write_all(kubeconfig_content.as_bytes())
        .expect("Failed to write kubeconfig content");

    // Set the KUBECONFIG environment variable to point to the new file
    env::set_var("KUBECONFIG", kubeconfig_path.to_str().unwrap());

    // Return the temporary directory and paths for use in tests
    let home_dir = temp_dir.path().to_str().unwrap().to_owned();
    let kubeconfig = kubeconfig_path.to_str().unwrap().to_owned();

    (temp_dir, home_dir, kubeconfig)
}

fn reset_environment() {
    env::remove_var("HOME");
    env::remove_var("KUBECONFIG");
}

#[test]
#[serial]
fn set_context() -> Result<(), Box<dyn std::error::Error>> {
    let (_temp_dir, _home_dir, kubeconfig) = setup_environment();

    let expected: String = format!("{}", kubeconfig);

    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("-v docker-desktop").arg("context").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert!(output_string.contains(&expected));
    reset_environment();

    Ok(())
}

#[test]
#[serial]
fn set_default_context() -> Result<(), Box<dyn std::error::Error>> {
    let (_temp_dir, _home_dir, kubeconfig) = setup_environment();

    let expected: String = format!("{}", kubeconfig);

    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v docker-desktop").arg("default-context").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert!(output_string.contains(&expected));

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn set_namespace() -> Result<(), Box<dyn std::error::Error>> {
    let (_temp_dir, _home_dir, kubeconfig) = setup_environment();
    let expected: String = format!("{}", kubeconfig);

    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v default").arg("namespace").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert!(output_string.contains(&expected));

    Ok(())
}

#[test]
#[serial]
fn set_default_namespace() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let (_temp_dir, _home_dir, kubeconfig) = setup_environment();
    let expected: String = format!("{}", kubeconfig);

    let mut cmd = Command::cargo_bin("kubesess")?;

    let output = cmd.arg("-v default").arg("default-namespace").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert!(output_string.contains(&expected));

    Ok(())
}
