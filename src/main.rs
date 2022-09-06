mod commands;
mod config;
mod model;
mod modes;

use clap::Parser;
use std::{env, io};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref KUBECONFIG: String = format!("{}/.kube/config", dirs::home_dir().unwrap().display());
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
    set_handlers();
    let args = Cli::parse();

    Mode::invoke(&args.mode);

    Ok(())
}

fn set_handlers() {
    ctrlc::set_handler(move || {
        println!("{}", env::var("KUBECONFIG").unwrap());
    })
    .expect("Error setting Ctrl-C handler");

    #[cfg(not(debug_assertions))]
    std::panic::set_hook(Box::new(move |_info| {
        std::process::exit(1);
    }));
}
