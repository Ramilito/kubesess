mod commands;
mod config;
mod error;
mod modes;

use crate::error::Error;
use clap::Parser;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::process;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref KUBECONFIG: String = {
        let mut paths_set: HashSet<String> = HashSet::new();

        // Get KUBECONFIG environment variable and add unique paths, ignoring kubesess/cache
        if let Ok(val) = env::var("KUBECONFIG") {
            val.split(':')
                .filter(|s| !s.contains("/kubesess/cache"))
                .for_each(|s| {
                    paths_set.insert(s.to_string());
                });
        }

        // Add all files under .kube directory
        if let Some(home_dir) = dirs::home_dir() {
            let kube_dir = home_dir.join(".kube");
            if let Ok(entries) = fs::read_dir(&kube_dir) {
                entries.filter_map(Result::ok).for_each(|entry| {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(path_str) = path.to_str() {
                            paths_set.insert(path_str.to_string());
                        }
                    }
                });
            }
        }

        // Collect all unique paths, sort them, and move .kube/config to the end if it exists
        let mut paths_vec: Vec<_> = paths_set.into_iter().collect();
        paths_vec.sort();
        if let Some(pos) = paths_vec.iter().position(|p| p.ends_with(".kube/config")) {
            let kube_config = paths_vec.remove(pos);
            paths_vec.push(kube_config);
        }

        paths_vec.join(":")
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

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = Cli::parse();

    if let Err(err) = Mode::invoke(&args.mode) {
        eprintln!("error: {}", err);
        process::exit(1);
    }

    Ok(())
}
