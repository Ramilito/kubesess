mod commands;
mod config;
mod error;
mod modes;

use crate::error::Error;
use clap::Parser;
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
                    .filter(|s| !s.contains("/kubesess/cache"))
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
    #[clap(value_enum, display_order = 1)]
    mode: Mode,
    #[clap(short, long, value_parser, display_order = 2)]
    value: Option<String>,
    #[clap(short, long, action)]
    current: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Namespace,
    Context,
    DefaultContext,
    DefaultNamespace,
    CompletionContext,
    CompletionNamespace,
}

impl Mode {
    fn invoke(&self) -> Result<(), Error> {
        let args = Cli::parse();
        match self {
            Mode::Namespace => modes::namespace(args),
            Mode::Context => modes::context(args),
            Mode::DefaultContext => modes::default_context(args),
            Mode::DefaultNamespace => modes::default_namespace(args),
            Mode::CompletionContext => {
                modes::completion_context(args);
                Ok(())
            }
            Mode::CompletionNamespace => {
                modes::completion_namespace(args);
                Ok(())
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();

    if let Err(err) = Mode::invoke(&args.mode) {
        eprintln!("error: {}", err);
        process::exit(1);
    }

    Ok(())
}
