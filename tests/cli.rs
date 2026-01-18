use assert_cmd::prelude::*;
use serial_test::serial;
use std::{env, fs, fs::File, io::Write, path::PathBuf, process::Command};
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
    let output = cmd.arg("context").arg("-v").arg("docker-desktop").unwrap();
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

    let output = cmd.arg("default-context").arg("-v").arg("docker-desktop").unwrap();
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

    let output = cmd.arg("namespace").arg("-v").arg("default").unwrap();
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

    let output = cmd.arg("default-namespace").arg("-v").arg("default").unwrap();
    let output_string = String::from_utf8(output.stdout).unwrap().trim().to_owned();

    assert!(output_string.contains(&expected));

    Ok(())
}

// =============================================================================
// Multi-Kubeconfig Test Infrastructure
// =============================================================================

struct MultiKubeconfigEnv {
    temp_dir: TempDir,
    kube_dir: PathBuf,
    config_path: PathBuf,
    work_path: PathBuf,
    personal_path: PathBuf,
}

fn create_kubeconfig_content(
    context_name: &str,
    cluster_name: &str,
    user_name: &str,
    namespace: &str,
    current_context: Option<&str>,
) -> String {
    let current_ctx_line = match current_context {
        Some(ctx) => format!("current-context: {}", ctx),
        None => String::new(),
    };

    format!(
        r#"apiVersion: v1
kind: Config
preferences: {{}}
{}
clusters:
- cluster:
    server: https://{}.example.com:6443
  name: {}
contexts:
- context:
    cluster: {}
    namespace: {}
    user: {}
  name: {}
users:
- name: {}
"#,
        current_ctx_line,
        cluster_name,
        cluster_name,
        cluster_name,
        namespace,
        user_name,
        context_name,
        user_name
    )
}

fn setup_multi_kubeconfig_environment() -> MultiKubeconfigEnv {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create ~/.kube directory structure
    let kube_dir = temp_dir.path().join(".kube");
    fs::create_dir_all(&kube_dir).expect("Failed to create .kube directory");

    // Create kubesess cache directory
    let cache_dir = kube_dir.join("kubesess").join("cache");
    fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");

    // Create main config file with docker-desktop context
    let config_path = kube_dir.join("config");
    let config_content = create_kubeconfig_content(
        "docker-desktop",
        "docker-desktop",
        "docker-desktop-user",
        "default",
        Some("docker-desktop"),
    );
    let mut config_file = File::create(&config_path).expect("Failed to create config");
    config_file
        .write_all(config_content.as_bytes())
        .expect("Failed to write config");

    // Create work.yaml with work-prod context
    let work_path = kube_dir.join("work.yaml");
    let work_content = create_kubeconfig_content(
        "work-prod",
        "work-cluster",
        "work-user",
        "production",
        Some("work-prod"),
    );
    let mut work_file = File::create(&work_path).expect("Failed to create work.yaml");
    work_file
        .write_all(work_content.as_bytes())
        .expect("Failed to write work.yaml");

    // Create personal.yaml with personal-dev context
    let personal_path = kube_dir.join("personal.yaml");
    let personal_content = create_kubeconfig_content(
        "personal-dev",
        "personal-cluster",
        "personal-user",
        "development",
        None, // No current-context set
    );
    let mut personal_file = File::create(&personal_path).expect("Failed to create personal.yaml");
    personal_file
        .write_all(personal_content.as_bytes())
        .expect("Failed to write personal.yaml");

    env::set_var("HOME", temp_dir.path());

    MultiKubeconfigEnv {
        temp_dir,
        kube_dir,
        config_path,
        work_path,
        personal_path,
    }
}

fn read_current_context_from_file(path: &PathBuf) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if line.starts_with("current-context:") {
            return Some(line.replace("current-context:", "").trim().to_string());
        }
    }
    None
}

// =============================================================================
// Scenario 1: Explicit KUBECONFIG with multiple files
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_explicit_list_contexts() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Set explicit KUBECONFIG with all three files
    let kubeconfig_value = format!(
        "{}:{}:{}",
        env.config_path.display(),
        env.work_path.display(),
        env.personal_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Test that we can select a context from any of the files
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-v").arg("work-prod").output()?;
    let output_string = String::from_utf8(output.stdout)?.trim().to_owned();

    // Should contain the cache path and the original KUBECONFIG
    assert!(
        output_string.contains("work-prod"),
        "Output should reference work-prod context: {}",
        output_string
    );

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn multi_kubeconfig_explicit_select_context_from_second_file() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Set explicit KUBECONFIG: config first, work second
    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Select context from the SECOND file (work.yaml)
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-v").arg("work-prod").output()?;
    let output_string = String::from_utf8(output.stdout)?.trim().to_owned();

    assert!(
        output_string.contains("work-prod"),
        "Should be able to select context from second file: {}",
        output_string
    );

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 2: Auto-discovery from ~/.kube/ (no KUBECONFIG set)
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_auto_discovery_finds_all_contexts() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Do NOT set KUBECONFIG - rely on auto-discovery
    std::env::remove_var("KUBECONFIG");

    // Try to select work-prod (from work.yaml, auto-discovered)
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-v").arg("work-prod").output()?;
    let output_string = String::from_utf8(output.stdout)?.trim().to_owned();
    let stderr = String::from_utf8(output.stderr)?;

    assert!(
        output.status.success(),
        "Command should succeed with auto-discovery. stderr: {}",
        stderr
    );
    assert!(
        output_string.contains("work-prod"),
        "Should find work-prod via auto-discovery: {}",
        output_string
    );

    // Also test personal-dev context
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("context").arg("-v").arg("personal-dev").output()?;
    let output_string2 = String::from_utf8(output2.stdout)?.trim().to_owned();

    assert!(
        output_string2.contains("personal-dev"),
        "Should find personal-dev via auto-discovery: {}",
        output_string2
    );

    // Keep temp_dir alive
    drop(env.temp_dir);
    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 3: default-context - which file gets modified?
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_default_context_modifies_which_file() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Record initial current-context values
    let initial_config_ctx = read_current_context_from_file(&env.config_path);
    let initial_work_ctx = read_current_context_from_file(&env.work_path);
    let initial_personal_ctx = read_current_context_from_file(&env.personal_path);

    println!("Initial states:");
    println!("  config: {:?}", initial_config_ctx);
    println!("  work.yaml: {:?}", initial_work_ctx);
    println!("  personal.yaml: {:?}", initial_personal_ctx);

    // Set explicit KUBECONFIG: config first, then work, then personal
    let kubeconfig_value = format!(
        "{}:{}:{}",
        env.config_path.display(),
        env.work_path.display(),
        env.personal_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Set default context to work-prod (defined in work.yaml, NOT in config)
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd
        .arg("default-context")
        .arg("-v")
        .arg("work-prod")
        .output()?;

    let stdout = String::from_utf8(output.stdout)?.trim().to_owned();
    let stderr = String::from_utf8(output.stderr)?;

    println!("Command output: {}", stdout);
    println!("Command stderr: {}", stderr);

    // Read current-context from all files after the command
    let after_config_ctx = read_current_context_from_file(&env.config_path);
    let after_work_ctx = read_current_context_from_file(&env.work_path);
    let after_personal_ctx = read_current_context_from_file(&env.personal_path);

    println!("After states:");
    println!("  config: {:?}", after_config_ctx);
    println!("  work.yaml: {:?}", after_work_ctx);
    println!("  personal.yaml: {:?}", after_personal_ctx);

    // Determine which file was modified
    let config_modified = initial_config_ctx != after_config_ctx;
    let work_modified = initial_work_ctx != after_work_ctx;
    let personal_modified = initial_personal_ctx != after_personal_ctx;

    println!("Files modified:");
    println!("  config: {}", config_modified);
    println!("  work.yaml: {}", work_modified);
    println!("  personal.yaml: {}", personal_modified);

    // CORRECT behavior:
    // - work.yaml contains work-prod context
    // - work.yaml already has current-context: work-prod
    // - So kubectl doesn't need to modify it (already correct)
    // - config should NOT be modified (it doesn't contain work-prod)

    // The key assertion: config must NOT be modified (that was the bug)
    assert!(
        !config_modified,
        "BUG: config was modified but it doesn't contain work-prod context"
    );

    // Document the behavior
    if !config_modified && !work_modified {
        println!("CORRECT: Neither file modified (work.yaml already had current-context: work-prod)");
    } else if work_modified && !config_modified {
        println!("CORRECT: Only work.yaml was modified (contains the context)");
    } else if config_modified {
        println!("BUG: config was modified (first in KUBECONFIG, but doesn't contain the context)");
    }

    // Verify work.yaml has the correct current-context
    assert_eq!(
        after_work_ctx,
        Some("work-prod".to_string()),
        "work.yaml should have current-context: work-prod"
    );

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn multi_kubeconfig_default_context_changes_correct_file() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Modify work.yaml to have a DIFFERENT current-context initially
    // This forces kubectl to actually change the file
    let work_content = create_kubeconfig_content(
        "work-prod",
        "work-cluster",
        "work-user",
        "production",
        Some("some-other-context"), // Different from work-prod
    );
    fs::write(&env.work_path, work_content)?;

    let initial_config_ctx = read_current_context_from_file(&env.config_path);
    let initial_work_ctx = read_current_context_from_file(&env.work_path);

    println!("Initial states (modified):");
    println!("  config: {:?}", initial_config_ctx);
    println!("  work.yaml: {:?}", initial_work_ctx);

    let kubeconfig_value = format!(
        "{}:{}:{}",
        env.config_path.display(),
        env.work_path.display(),
        env.personal_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Set default context to work-prod
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd
        .arg("default-context")
        .arg("-v")
        .arg("work-prod")
        .output()?;

    let stdout = String::from_utf8(output.stdout)?.trim().to_owned();
    let stderr = String::from_utf8(output.stderr)?;

    println!("Output: {}", stdout);
    if !stderr.is_empty() {
        println!("Stderr: {}", stderr);
    }

    let after_config_ctx = read_current_context_from_file(&env.config_path);
    let after_work_ctx = read_current_context_from_file(&env.work_path);

    println!("After states:");
    println!("  config: {:?}", after_config_ctx);
    println!("  work.yaml: {:?}", after_work_ctx);

    let config_modified = initial_config_ctx != after_config_ctx;
    let work_modified = initial_work_ctx != after_work_ctx;

    println!("Files modified - config: {}, work: {}", config_modified, work_modified);

    // CORRECT behavior: work.yaml should be modified, config should NOT
    assert!(
        !config_modified,
        "BUG: config was modified but it doesn't contain work-prod context"
    );
    assert!(
        work_modified,
        "work.yaml should have been modified to set current-context: work-prod"
    );
    assert_eq!(
        after_work_ctx,
        Some("work-prod".to_string()),
        "work.yaml should now have current-context: work-prod"
    );

    println!("CORRECT: Only work.yaml was modified (contains the context)");

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn multi_kubeconfig_default_context_with_auto_discovery() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Record initial states
    let initial_config_ctx = read_current_context_from_file(&env.config_path);
    let initial_work_ctx = read_current_context_from_file(&env.work_path);

    println!("Initial (auto-discovery):");
    println!("  config: {:?}", initial_config_ctx);
    println!("  work.yaml: {:?}", initial_work_ctx);

    // NO KUBECONFIG set - use auto-discovery
    std::env::remove_var("KUBECONFIG");

    // Set default context to work-prod
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd
        .arg("default-context")
        .arg("-v")
        .arg("work-prod")
        .output()?;

    let stdout = String::from_utf8(output.stdout)?.trim().to_owned();
    let stderr = String::from_utf8(output.stderr)?;

    println!("Output: {}", stdout);
    if !stderr.is_empty() {
        println!("Stderr: {}", stderr);
    }

    // Read after states
    let after_config_ctx = read_current_context_from_file(&env.config_path);
    let after_work_ctx = read_current_context_from_file(&env.work_path);

    println!("After (auto-discovery):");
    println!("  config: {:?}", after_config_ctx);
    println!("  work.yaml: {:?}", after_work_ctx);

    let config_modified = initial_config_ctx != after_config_ctx;
    let work_modified = initial_work_ctx != after_work_ctx;

    println!("Modified - config: {}, work: {}", config_modified, work_modified);

    // With auto-discovery, the order is non-deterministic
    // This test documents what actually happens
    if !config_modified && !work_modified {
        println!("NOTE: Neither file was modified - kubectl might not be available in test");
    }

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 4: default-context after session context (cache involved)
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_session_then_default_context() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Step 1: Set session context to docker-desktop
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let output1 = cmd1.arg("context").arg("-v").arg("docker-desktop").output()?;
    let session_output = String::from_utf8(output1.stdout)?.trim().to_owned();

    println!("Session context output: {}", session_output);

    // The output should contain a cache path
    assert!(
        session_output.contains("kubesess/cache"),
        "Session output should include cache path: {}",
        session_output
    );

    // Step 2: Now simulate what the shell would do - set KUBECONFIG to the output
    std::env::set_var("KUBECONFIG", &session_output);

    // Step 3: Set default context to work-prod
    let initial_config_ctx = read_current_context_from_file(&env.config_path);
    let initial_work_ctx = read_current_context_from_file(&env.work_path);

    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2
        .arg("default-context")
        .arg("-v")
        .arg("work-prod")
        .output()?;

    let default_output = String::from_utf8(output2.stdout)?.trim().to_owned();
    let stderr = String::from_utf8(output2.stderr)?;

    println!("Default context output: {}", default_output);
    if !stderr.is_empty() {
        println!("Stderr: {}", stderr);
    }

    let after_config_ctx = read_current_context_from_file(&env.config_path);
    let after_work_ctx = read_current_context_from_file(&env.work_path);

    println!("File states after default-context:");
    println!("  config: {:?} -> {:?}", initial_config_ctx, after_config_ctx);
    println!("  work.yaml: {:?} -> {:?}", initial_work_ctx, after_work_ctx);

    // Check if cache file was modified instead of the real files
    // (This would be a bug)
    let cache_dir = env.kube_dir.join("kubesess").join("cache");
    if cache_dir.exists() {
        let cache_files: Vec<_> = fs::read_dir(&cache_dir)?
            .filter_map(Result::ok)
            .collect();
        println!("Cache files: {:?}", cache_files.iter().map(|e| e.path()).collect::<Vec<_>>());
    }

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 5: Verify output contains correct file ordering
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_output_format_explicit() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Set default context to work-prod
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd
        .arg("default-context")
        .arg("-v")
        .arg("work-prod")
        .output()?;

    let stdout = String::from_utf8(output.stdout)?.trim().to_owned();

    println!("Output for work-prod: {}", stdout);

    // The output should have work.yaml first (since it contains work-prod)
    // followed by the rest of KUBECONFIG
    let parts: Vec<&str> = stdout.split(':').collect();

    println!("Output parts:");
    for (i, part) in parts.iter().enumerate() {
        println!("  [{}]: {}", i, part);
    }

    // First part should be the file containing the selected context
    if !parts.is_empty() {
        let first_part = parts[0];
        if first_part.contains("work") {
            println!("CORRECT: work.yaml is first in output");
        } else if first_part.contains("config") {
            println!("NOTE: config is first in output (might be expected based on KUBECONFIG order)");
        }
    }

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn multi_kubeconfig_session_output_format() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Set session context to work-prod
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-v").arg("work-prod").output()?;

    let stdout = String::from_utf8(output.stdout)?.trim().to_owned();

    println!("Session output for work-prod: {}", stdout);

    let parts: Vec<&str> = stdout.split(':').collect();

    println!("Session output parts:");
    for (i, part) in parts.iter().enumerate() {
        println!("  [{}]: {}", i, part);
    }

    // First part should be the cache file
    if !parts.is_empty() {
        let first_part = parts[0];
        if first_part.contains("kubesess/cache") {
            println!("CORRECT: Cache file is first in session output");
        } else {
            println!("UNEXPECTED: First part is not cache file: {}", first_part);
        }
    }

    // Remaining parts should be the original KUBECONFIG (without cache)
    let remaining: Vec<&str> = parts.iter().skip(1).cloned().collect();
    println!("Remaining (original KUBECONFIG): {:?}", remaining);

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 6: Current context with multiple files
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_current_context_first_file_wins() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // config has current-context: docker-desktop
    // work.yaml has current-context: work-prod
    // Order: config first
    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-c").output()?;
    let current = String::from_utf8(output.stdout)?.trim().to_owned();

    println!("Current context (config first): {}", current);
    assert_eq!(current, "docker-desktop", "First file's current-context should win");

    // Now reverse the order: work first
    let kubeconfig_value2 = format!(
        "{}:{}",
        env.work_path.display(),
        env.config_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value2);

    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("context").arg("-c").output()?;
    let current2 = String::from_utf8(output2.stdout)?.trim().to_owned();

    println!("Current context (work first): {}", current2);
    assert_eq!(current2, "work-prod", "First file's current-context should win");

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 7: default-namespace - which file gets modified?
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_default_namespace_modifies_which_file() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Set explicit KUBECONFIG: config first, work second
    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Read initial file contents
    let initial_config = fs::read_to_string(&env.config_path)?;
    let initial_work = fs::read_to_string(&env.work_path)?;

    println!("Initial config namespace: {}",
        initial_config.lines().find(|l| l.contains("namespace:")).unwrap_or("not found"));
    println!("Initial work namespace: {}",
        initial_work.lines().find(|l| l.contains("namespace:")).unwrap_or("not found"));

    // Set default namespace on work-prod context (which is in work.yaml)
    // First we need to be "in" the work-prod context
    // Run session context first to set current context
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let output1 = cmd1.arg("context").arg("-v").arg("work-prod").output()?;
    let session_output = String::from_utf8(output1.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &session_output);

    // Now set default namespace
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2
        .arg("default-namespace")
        .arg("-v")
        .arg("new-namespace")
        .output()?;

    let stdout = String::from_utf8(output2.stdout)?.trim().to_owned();
    let stderr = String::from_utf8(output2.stderr)?;

    println!("Default namespace output: {}", stdout);
    if !stderr.is_empty() {
        println!("Stderr: {}", stderr);
    }

    // Read files after
    let after_config = fs::read_to_string(&env.config_path)?;
    let after_work = fs::read_to_string(&env.work_path)?;

    let config_modified = initial_config != after_config;
    let work_modified = initial_work != after_work;

    println!("After config namespace: {}",
        after_config.lines().find(|l| l.contains("namespace:")).unwrap_or("not found"));
    println!("After work namespace: {}",
        after_work.lines().find(|l| l.contains("namespace:")).unwrap_or("not found"));

    println!("Files modified - config: {}, work: {}", config_modified, work_modified);

    // Expected: work.yaml should be modified (contains work-prod context)
    if work_modified && !config_modified {
        println!("CORRECT: work.yaml was modified (contains the context)");
    } else if config_modified && !work_modified {
        println!("BUG: config was modified instead of work.yaml");
    } else if config_modified && work_modified {
        println!("BOTH files were modified");
    } else {
        println!("NEITHER file was modified");
    }

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 8: Sequential context switches
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_sequential_session_context_switches() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}:{}",
        env.config_path.display(),
        env.work_path.display(),
        env.personal_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Switch to docker-desktop
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let output1 = cmd1.arg("context").arg("-v").arg("docker-desktop").output()?;
    let kubeconfig1 = String::from_utf8(output1.stdout)?.trim().to_owned();
    println!("After switch to docker-desktop: {}", kubeconfig1);

    // Simulate shell export
    std::env::set_var("KUBECONFIG", &kubeconfig1);

    // Verify current context
    let mut cmd1c = Command::cargo_bin("kubesess")?;
    let current1 = String::from_utf8(cmd1c.arg("context").arg("-c").output()?.stdout)?.trim().to_owned();
    println!("Current context after switch 1: {}", current1);
    assert_eq!(current1, "docker-desktop");

    // Switch to work-prod
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("context").arg("-v").arg("work-prod").output()?;
    let kubeconfig2 = String::from_utf8(output2.stdout)?.trim().to_owned();
    println!("After switch to work-prod: {}", kubeconfig2);

    std::env::set_var("KUBECONFIG", &kubeconfig2);

    let mut cmd2c = Command::cargo_bin("kubesess")?;
    let current2 = String::from_utf8(cmd2c.arg("context").arg("-c").output()?.stdout)?.trim().to_owned();
    println!("Current context after switch 2: {}", current2);
    assert_eq!(current2, "work-prod");

    // Switch to personal-dev
    let mut cmd3 = Command::cargo_bin("kubesess")?;
    let output3 = cmd3.arg("context").arg("-v").arg("personal-dev").output()?;
    let kubeconfig3 = String::from_utf8(output3.stdout)?.trim().to_owned();
    println!("After switch to personal-dev: {}", kubeconfig3);

    std::env::set_var("KUBECONFIG", &kubeconfig3);

    let mut cmd3c = Command::cargo_bin("kubesess")?;
    let current3 = String::from_utf8(cmd3c.arg("context").arg("-c").output()?.stdout)?.trim().to_owned();
    println!("Current context after switch 3: {}", current3);
    assert_eq!(current3, "personal-dev");

    // Count cache files created
    let cache_dir = env.kube_dir.join("kubesess").join("cache");
    let cache_files: Vec<_> = fs::read_dir(&cache_dir)?
        .filter_map(Result::ok)
        .collect();
    println!("Cache files created: {}", cache_files.len());
    for f in &cache_files {
        println!("  - {}", f.path().display());
    }

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn multi_kubeconfig_sequential_default_context_switches() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Initial state
    let initial_config_ctx = read_current_context_from_file(&env.config_path);
    let initial_work_ctx = read_current_context_from_file(&env.work_path);
    println!("Initial - config: {:?}, work: {:?}", initial_config_ctx, initial_work_ctx);

    // Set default to work-prod
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let output1 = cmd1.arg("default-context").arg("-v").arg("work-prod").output()?;
    let kubeconfig1 = String::from_utf8(output1.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &kubeconfig1);

    let after1_config_ctx = read_current_context_from_file(&env.config_path);
    let after1_work_ctx = read_current_context_from_file(&env.work_path);
    println!("After work-prod - config: {:?}, work: {:?}", after1_config_ctx, after1_work_ctx);

    // Set default to docker-desktop
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("default-context").arg("-v").arg("docker-desktop").output()?;
    let kubeconfig2 = String::from_utf8(output2.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &kubeconfig2);

    let after2_config_ctx = read_current_context_from_file(&env.config_path);
    let after2_work_ctx = read_current_context_from_file(&env.work_path);
    println!("After docker-desktop - config: {:?}, work: {:?}", after2_config_ctx, after2_work_ctx);

    // Set default back to work-prod
    let mut cmd3 = Command::cargo_bin("kubesess")?;
    let output3 = cmd3.arg("default-context").arg("-v").arg("work-prod").output()?;
    let kubeconfig3 = String::from_utf8(output3.stdout)?.trim().to_owned();

    let after3_config_ctx = read_current_context_from_file(&env.config_path);
    let after3_work_ctx = read_current_context_from_file(&env.work_path);
    println!("After work-prod again - config: {:?}, work: {:?}", after3_config_ctx, after3_work_ctx);

    println!("Final output: {}", kubeconfig3);

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 9: Cache file contents validation
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_cache_file_is_valid_kubeconfig() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Create a session context
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-v").arg("work-prod").output()?;
    let kubeconfig_output = String::from_utf8(output.stdout)?.trim().to_owned();

    // Extract cache file path (first in the output)
    let cache_path = kubeconfig_output.split(':').next().unwrap();
    println!("Cache file: {}", cache_path);

    // Read and validate cache file
    let cache_content = fs::read_to_string(cache_path)?;
    println!("Cache content:\n{}", cache_content);

    // Document what fields are present (not all kubeconfig fields may be serialized)
    println!("Has apiVersion: {}", cache_content.contains("apiVersion:"));
    println!("Has kind: {}", cache_content.contains("kind:"));
    println!("Has current-context: {}", cache_content.contains("current-context:"));
    println!("Has work-prod: {}", cache_content.contains("work-prod"));
    println!("Has work-cluster: {}", cache_content.contains("work-cluster"));
    println!("Has work-user: {}", cache_content.contains("work-user"));

    // These are essential for the cache to work
    assert!(cache_content.contains("current-context:"), "Should have current-context");
    assert!(cache_content.contains("work-prod"), "Should reference work-prod");
    assert!(cache_content.contains("work-cluster"), "Should contain the cluster");
    assert!(cache_content.contains("work-user"), "Should contain the user");

    // Verify it's minimal (only one context)
    let context_count = cache_content.matches("- name:").count();
    println!("Number of named entries: {}", context_count);

    // Note: Cache files may not have apiVersion/kind if using serde_yaml directly
    if !cache_content.contains("apiVersion:") {
        println!("NOTE: Cache file missing apiVersion - may cause issues with some kubectl versions");
    }

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn multi_kubeconfig_cache_contains_correct_namespace() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!("{}", env.work_path.display());
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Set namespace
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let output1 = cmd1.arg("context").arg("-v").arg("work-prod").output()?;
    let kubeconfig1 = String::from_utf8(output1.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &kubeconfig1);

    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("namespace").arg("-v").arg("custom-ns").output()?;
    let kubeconfig2 = String::from_utf8(output2.stdout)?.trim().to_owned();

    // Extract cache path
    let cache_path = kubeconfig2.split(':').next().unwrap();
    println!("Cache file for namespace test: {}", cache_path);

    let cache_content = fs::read_to_string(cache_path)?;
    println!("Cache content:\n{}", cache_content);

    assert!(cache_content.contains("namespace: custom-ns"),
        "Cache should contain custom-ns namespace");

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 10: File integrity after operations
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_files_remain_valid_after_default_operations() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Perform several default operations
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    cmd1.arg("default-context").arg("-v").arg("work-prod").output()?;

    let mut cmd2 = Command::cargo_bin("kubesess")?;
    cmd2.arg("default-context").arg("-v").arg("docker-desktop").output()?;

    // Verify both files are still valid YAML
    let config_content = fs::read_to_string(&env.config_path)?;
    let work_content = fs::read_to_string(&env.work_path)?;

    println!("Config after operations:\n{}", config_content);
    println!("Work after operations:\n{}", work_content);

    // Basic validation - should still have required fields
    assert!(config_content.contains("apiVersion:"), "config should have apiVersion");
    assert!(config_content.contains("clusters:"), "config should have clusters");
    assert!(config_content.contains("contexts:"), "config should have contexts");

    assert!(work_content.contains("apiVersion:"), "work should have apiVersion");
    assert!(work_content.contains("clusters:"), "work should have clusters");
    assert!(work_content.contains("contexts:"), "work should have contexts");

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 11: Edge cases
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_single_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // Only set config (single file)
    std::env::set_var("KUBECONFIG", env.config_path.to_str().unwrap());

    // Session context
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let output1 = cmd1.arg("context").arg("-v").arg("docker-desktop").output()?;
    let stdout1 = String::from_utf8(output1.stdout)?.trim().to_owned();
    println!("Single file - session context output: {}", stdout1);
    assert!(output1.status.success());

    std::env::set_var("KUBECONFIG", &stdout1);

    // Default context
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("default-context").arg("-v").arg("docker-desktop").output()?;
    let stdout2 = String::from_utf8(output2.stdout)?.trim().to_owned();
    println!("Single file - default context output: {}", stdout2);
    assert!(output2.status.success());

    // Current context query
    let mut cmd3 = Command::cargo_bin("kubesess")?;
    let output3 = cmd3.arg("context").arg("-c").output()?;
    let current = String::from_utf8(output3.stdout)?.trim().to_owned();
    println!("Single file - current context: {}", current);
    assert_eq!(current, "docker-desktop");

    reset_environment();
    Ok(())
}

#[test]
#[serial]
fn multi_kubeconfig_nonexistent_context_error() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!("{}", env.config_path.display());
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Try to switch to non-existent context
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-v").arg("nonexistent-context").output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    println!("Nonexistent context stdout: {}", stdout);
    println!("Nonexistent context stderr: {}", stderr);
    println!("Exit status: {:?}", output.status);

    // Document actual behavior
    if !output.status.success() {
        println!("CORRECT: Command failed for nonexistent context");
    } else if stderr.contains("error") || stderr.contains("not found") || stderr.contains("Error") {
        println!("OK: Command succeeded but printed error message");
    } else if stdout.is_empty() {
        println!("OK: Command produced no output for nonexistent context");
    } else {
        println!("POTENTIAL ISSUE: Command succeeded without error for nonexistent context");
        println!("  stdout: {}", stdout);
    }

    // We're documenting behavior, not enforcing it strictly
    // The test passes as long as it doesn't panic

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 12: Namespace operations across files
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_namespace_with_context_from_different_file() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Switch to work-prod context (from work.yaml)
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let output1 = cmd1.arg("context").arg("-v").arg("work-prod").output()?;
    let kubeconfig1 = String::from_utf8(output1.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &kubeconfig1);

    println!("After context switch: {}", kubeconfig1);

    // Get current namespace
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("namespace").arg("-c").output()?;
    let current_ns = String::from_utf8(output2.stdout)?.trim().to_owned();
    println!("Current namespace: {}", current_ns);

    // Set namespace (session)
    let mut cmd3 = Command::cargo_bin("kubesess")?;
    let output3 = cmd3.arg("namespace").arg("-v").arg("test-namespace").output()?;
    let kubeconfig3 = String::from_utf8(output3.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &kubeconfig3);

    println!("After namespace switch: {}", kubeconfig3);

    // Verify namespace changed
    let mut cmd4 = Command::cargo_bin("kubesess")?;
    let output4 = cmd4.arg("namespace").arg("-c").output()?;
    let new_ns = String::from_utf8(output4.stdout)?.trim().to_owned();
    println!("New namespace: {}", new_ns);

    assert_eq!(new_ns, "test-namespace");

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 13: Output path deduplication
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_output_should_not_have_duplicates() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}:{}",
        env.config_path.display(),
        env.work_path.display(),
        env.personal_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Set default context to work-prod
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("default-context").arg("-v").arg("work-prod").output()?;
    let stdout = String::from_utf8(output.stdout)?.trim().to_owned();

    println!("Output: {}", stdout);

    let parts: Vec<&str> = stdout.split(':').collect();
    let unique_parts: std::collections::HashSet<&str> = parts.iter().cloned().collect();

    println!("Total parts: {}", parts.len());
    println!("Unique parts: {}", unique_parts.len());

    if parts.len() != unique_parts.len() {
        println!("ISSUE: Output contains duplicate paths!");
        for (i, part) in parts.iter().enumerate() {
            let count = parts.iter().filter(|p| *p == part).count();
            if count > 1 {
                println!("  [{}] {} (appears {} times)", i, part, count);
            }
        }
    } else {
        println!("OK: No duplicate paths in output");
    }

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 14: Session then default - verify correct file modified
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_session_does_not_affect_source_files() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Record initial file contents
    let initial_config = fs::read_to_string(&env.config_path)?;
    let initial_work = fs::read_to_string(&env.work_path)?;

    // Perform multiple session operations
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let out1 = cmd1.arg("context").arg("-v").arg("work-prod").output()?;
    std::env::set_var("KUBECONFIG", String::from_utf8(out1.stdout)?.trim());

    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let out2 = cmd2.arg("namespace").arg("-v").arg("new-ns").output()?;
    std::env::set_var("KUBECONFIG", String::from_utf8(out2.stdout)?.trim());

    let mut cmd3 = Command::cargo_bin("kubesess")?;
    let out3 = cmd3.arg("context").arg("-v").arg("docker-desktop").output()?;
    std::env::set_var("KUBECONFIG", String::from_utf8(out3.stdout)?.trim());

    // Verify source files are unchanged
    let after_config = fs::read_to_string(&env.config_path)?;
    let after_work = fs::read_to_string(&env.work_path)?;

    assert_eq!(initial_config, after_config, "config should be unchanged after session operations");
    assert_eq!(initial_work, after_work, "work.yaml should be unchanged after session operations");

    println!("CORRECT: Session operations do not modify source files");

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 15: Verify context -c returns correct value after operations
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_current_context_accuracy_after_operations() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}",
        env.config_path.display(),
        env.work_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    // Initial current context
    let mut cmd0 = Command::cargo_bin("kubesess")?;
    let current0 = String::from_utf8(cmd0.arg("context").arg("-c").output()?.stdout)?.trim().to_owned();
    println!("Initial current context: {}", current0);

    // Session switch to work-prod
    let mut cmd1 = Command::cargo_bin("kubesess")?;
    let out1 = cmd1.arg("context").arg("-v").arg("work-prod").output()?;
    let kc1 = String::from_utf8(out1.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &kc1);

    let mut cmd1c = Command::cargo_bin("kubesess")?;
    let current1 = String::from_utf8(cmd1c.arg("context").arg("-c").output()?.stdout)?.trim().to_owned();
    println!("After session work-prod: {} (KUBECONFIG={})", current1, kc1);
    assert_eq!(current1, "work-prod", "Current should be work-prod after session switch");

    // Default switch to docker-desktop (should modify file)
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let out2 = cmd2.arg("default-context").arg("-v").arg("docker-desktop").output()?;
    let kc2 = String::from_utf8(out2.stdout)?.trim().to_owned();
    std::env::set_var("KUBECONFIG", &kc2);

    let mut cmd2c = Command::cargo_bin("kubesess")?;
    let current2 = String::from_utf8(cmd2c.arg("context").arg("-c").output()?.stdout)?.trim().to_owned();
    println!("After default docker-desktop: {} (KUBECONFIG={})", current2, kc2);

    // Note: This might not be docker-desktop if the cache from previous session is still first
    // This documents the actual behavior
    println!("Expected: docker-desktop, Actual: {}", current2);

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 16: Auto-discovery consistency (run multiple times)
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_auto_discovery_produces_consistent_order() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    // No KUBECONFIG set - use auto-discovery
    std::env::remove_var("KUBECONFIG");

    let mut outputs = Vec::new();

    // Run multiple times
    for i in 0..5 {
        let mut cmd = Command::cargo_bin("kubesess")?;
        let output = cmd.arg("context").arg("-v").arg("docker-desktop").output()?;
        let stdout = String::from_utf8(output.stdout)?.trim().to_owned();
        println!("Run {}: {}", i, stdout);
        outputs.push(stdout);
    }

    // Check if all outputs are the same
    let first = &outputs[0];
    let all_same = outputs.iter().all(|o| o == first);

    if all_same {
        println!("CONSISTENT: All runs produced the same output");
    } else {
        println!("INCONSISTENT: Different runs produced different outputs!");
        for (i, output) in outputs.iter().enumerate() {
            println!("  Run {}: {}", i, output);
        }
    }

    // This documents the behavior - we're not asserting here since we want to see what happens
    drop(env.temp_dir);
    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 17: Completion commands with multiple files
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_completion_context_lists_all() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}:{}",
        env.config_path.display(),
        env.work_path.display(),
        env.personal_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("completion-context").output()?;
    let stdout = String::from_utf8(output.stdout)?.trim().to_owned();

    println!("Completion output: {}", stdout);

    // Should list all contexts from all files
    assert!(stdout.contains("docker-desktop"), "Should list docker-desktop");
    assert!(stdout.contains("work-prod"), "Should list work-prod");
    assert!(stdout.contains("personal-dev"), "Should list personal-dev");

    reset_environment();
    Ok(())
}

// =============================================================================
// Scenario 18: Mixed operations stress test
// =============================================================================

#[test]
#[serial]
fn multi_kubeconfig_mixed_operations_stress_test() -> Result<(), Box<dyn std::error::Error>> {
    reset_environment();
    let env = setup_multi_kubeconfig_environment();

    let kubeconfig_value = format!(
        "{}:{}:{}",
        env.config_path.display(),
        env.work_path.display(),
        env.personal_path.display()
    );
    std::env::set_var("KUBECONFIG", &kubeconfig_value);

    let operations = vec![
        ("context", "-v", "docker-desktop"),
        ("context", "-v", "work-prod"),
        ("namespace", "-v", "ns1"),
        ("context", "-v", "personal-dev"),
        ("namespace", "-v", "ns2"),
        ("context", "-v", "docker-desktop"),
        ("namespace", "-v", "ns3"),
    ];

    for (cmd_name, flag, value) in &operations {
        let mut cmd = Command::cargo_bin("kubesess")?;
        let output = cmd.arg(*cmd_name).arg(*flag).arg(*value).output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?.trim().to_owned();
            std::env::set_var("KUBECONFIG", &stdout);
            println!("{} {} {} -> OK", cmd_name, flag, value);
        } else {
            let stderr = String::from_utf8(output.stderr)?;
            println!("{} {} {} -> FAILED: {}", cmd_name, flag, value, stderr);
        }
    }

    // Final current context check
    let mut cmd = Command::cargo_bin("kubesess")?;
    let output = cmd.arg("context").arg("-c").output()?;
    let current = String::from_utf8(output.stdout)?.trim().to_owned();
    println!("Final current context: {}", current);
    assert_eq!(current, "docker-desktop", "Final context should be docker-desktop");

    // Final current namespace check
    let mut cmd2 = Command::cargo_bin("kubesess")?;
    let output2 = cmd2.arg("namespace").arg("-c").output()?;
    let current_ns = String::from_utf8(output2.stdout)?.trim().to_owned();
    println!("Final current namespace: {}", current_ns);
    assert_eq!(current_ns, "ns3", "Final namespace should be ns3");

    reset_environment();
    Ok(())
}
