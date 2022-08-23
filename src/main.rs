mod commands;
mod config;
mod modes;
mod model;

use clap::Parser;
use std::{env, io};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(value_enum, display_order = 1)]
    mode: Mode,
    #[clap(short, long, value_parser, display_order = 2)]
    value: Option<String>,
}

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Namespace,
    Context,
    DefaultContext,
    DefaultNamespace,
}

impl Mode {
    fn invoke(&self, dest: &String) {
        match self {
            Mode::Namespace => modes::namespace(Cli::parse(), dest),
            Mode::Context => modes::context(Cli::parse(), dest),
            Mode::DefaultContext => modes::default_context(Cli::parse(), dest),
            Mode::DefaultNamespace => modes::default_namespace(Cli::parse(), dest),
        }
    }
}

fn main() -> Result<(), io::Error> {
    set_handlers();

    let args = Cli::parse();
    let dest = format!(
        "{}/.kube/kubesess/cache",
        dirs::home_dir().unwrap().display()
    );

    Mode::invoke(&args.mode, &dest);

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
