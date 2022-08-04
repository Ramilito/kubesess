mod commands;
mod config;
mod modes;

use clap::Parser;
use std::io;

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
}

impl Mode {
    fn invoke(&self, dest: &String) {
        match self {
            Mode::Namespace => modes::namespace(Cli::parse(), dest),
            Mode::Context => modes::context(Cli::parse(), dest),
            Mode::DefaultContext => modes::default_context(Cli::parse()),
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let dest = format!("{}/.kube/kubesess/cache", dirs::home_dir().unwrap().display());

    Mode::invoke(&args.mode, &dest);

    Ok(())
}
