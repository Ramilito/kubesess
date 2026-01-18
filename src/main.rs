mod commands;
mod config;
mod error;
mod init;
mod modes;

use clap::{Parser, Subcommand};
use kube::config::Kubeconfig;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref KUBECONFIG: String = {
        let mut paths_set = HashSet::new();
        let mut all_paths = Vec::new();

        // Get paths from KUBECONFIG environment variable, preserving order
        if let Ok(val) = env::var("KUBECONFIG") {
            all_paths.extend(
                val.split(':')
                    .filter(|s| !s.is_empty() && !s.contains("/kubesess/cache"))
                    .filter_map(|s| {
                        if paths_set.insert(s.to_string()) {
                            Some(s.to_string())
                        } else {
                            None
                        }
                    }),
            );
        }

        // Collect paths from ~/.kube directory
        if let Some(home_dir) = dirs::home_dir() {
            let kube_dir = home_dir.join(".kube");

            if let Ok(entries) = fs::read_dir(&kube_dir) {
                let kube_paths: Vec<String> = entries
                    .filter_map(Result::ok)
                    .map(|e| e.path())
                    .filter(|path: &PathBuf| {
                        // 1) must be a regular file
                        // 2) we haven’t added it yet
                        // 3) it parses as a kubeconfig
                        path.is_file()
                            && paths_set.insert(path.to_string_lossy().into_owned())
                            && Kubeconfig::read_from(path).is_ok()
                    })
                    .map(|p: PathBuf| p.to_string_lossy().into_owned())
                    .collect();

                all_paths.extend(kube_paths);
            }
        }

        all_paths.join(":")
    };
    static ref KUBESESSCONFIG: String = {
        match env::var("KUBECONFIG") {
            Ok(val) => {
                let mut paths: String = String::new();
                for s in val.split(':') {
                    if s.contains("/kubesess/cache") {
                        paths.push_str(s);
                    }
                }
                paths
            }
            Err(_e) => "".to_string(),
        }
    };
    static ref DEST: String = format!(
        "{}/.kube/kubesess/cache",
        dirs::home_dir().unwrap().display()
    );
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

/// Common arguments for context/namespace operations
#[derive(clap::Args, Clone)]
pub struct ModeArgs {
    /// Specify the value directly instead of using interactive selection
    #[clap(short, long, value_parser)]
    pub value: Option<String>,
    /// Print the current context/namespace
    #[clap(short, long, action)]
    pub current: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Switch to a context (session-specific)
    Context {
        #[clap(flatten)]
        args: ModeArgs,
    },
    /// Switch to a namespace (session-specific)
    Namespace {
        #[clap(flatten)]
        args: ModeArgs,
    },
    /// Switch to a context (global, modifies kubeconfig)
    DefaultContext {
        #[clap(flatten)]
        args: ModeArgs,
    },
    /// Switch to a namespace (global, modifies kubeconfig)
    DefaultNamespace {
        #[clap(flatten)]
        args: ModeArgs,
    },
    /// Output completions for context
    CompletionContext {
        #[clap(flatten)]
        args: ModeArgs,
    },
    /// Output completions for namespace
    CompletionNamespace {
        #[clap(flatten)]
        args: ModeArgs,
    },
    /// Initialize shell integration
    Init {
        /// Shell to generate initialization script for
        #[clap(value_enum)]
        shell: init::Shell,
    },
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Context { args } => modes::context(args),
        Command::Namespace { args } => modes::namespace(args),
        Command::DefaultContext { args } => modes::default_context(args),
        Command::DefaultNamespace { args } => modes::default_namespace(args),
        Command::CompletionContext { args } => {
            modes::completion_context(args);
            Ok(())
        }
        Command::CompletionNamespace { args } => {
            modes::completion_namespace(args);
            Ok(())
        }
        Command::Init { shell } => {
            init::print_init_script(shell);
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("error: {}", err);
        process::exit(1);
    }

    Ok(())
}
