mod commands;
mod config;
mod model;
mod modes;

use clap::Parser;
use std::env;
use std::io;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref KUBECONFIG: String = {
        match env::var("KUBECONFIG") {
            Ok(val) => {
                let mut paths: String = String::new();
                for s in val.split_inclusive(':') {
                    if s.contains("/kubesess/cache") {
                        continue;
                    }
                    paths.push_str(s);
                }
                paths
            }
            Err(_e) => format!("{}/.kube/config", dirs::home_dir().unwrap().display()),
        }
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
    fn invoke(&self) {
        match self {
            Mode::Namespace => modes::namespace(Cli::parse()),
            Mode::Context => modes::context(Cli::parse()),
            Mode::DefaultContext => modes::default_context(Cli::parse()),
            Mode::DefaultNamespace => modes::default_namespace(Cli::parse()),
            Mode::CompletionContext => modes::completion_context(Cli::parse()),
            Mode::CompletionNamespace => modes::completion_namespace(Cli::parse()),
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();

    Mode::invoke(&args.mode);

    Ok(())
}
