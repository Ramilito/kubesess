mod commands;

use clap::Parser;
use std::io;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_enum)]
    mode: Mode,

    #[clap(short, long, value_parser)]
    context: Option<String>,

    #[clap(short, long, value_parser)]
    namespace: Option<String>,
}

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Namespace,
    Context,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let temp_dir = format!("{}/.cache/kubesess", dirs::home_dir().unwrap().display());
    let selection;
    let ns;

    match args.mode {
        Mode::Namespace => {
            if let Some(x) = args.namespace{
                ns = x;
            } else {
                let contexts = commands::get_namespace();
                ns = commands::selectable_list(contexts);
            }

            selection = commands::get_current_context();
            commands::set_namespace(&selection, &ns, &temp_dir);
        }
        Mode::Context => {
            if args.context.is_some() {
                selection = args.context.unwrap().to_string();
            } else {
                let contexts = commands::get_context();
                selection = commands::selectable_list(contexts);
            }

            commands::set_context(&selection, &temp_dir);
        }
    }
    println!("{}/{}", &temp_dir, str::replace(&selection, ":", "_"));

    Ok(())
}
