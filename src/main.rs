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
    fn invoke(&self, temp_dir: &String) {
        match self {
            Mode::Namespace => modes::namespace(Cli::parse(), temp_dir),
            Mode::Context => modes::context(Cli::parse(), temp_dir),
            Mode::DefaultContext => modes::default_context(Cli::parse()),
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let temp_dir = format!("{}/.cache/kubesess", dirs::home_dir().unwrap().display());

    Mode::invoke(&args.mode, &temp_dir);

    Ok(())
}
