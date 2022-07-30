mod commands;

use clap::Parser;
use std::io;

#[derive(Parser)]
struct Cli {
    #[clap(value_enum)]
    mode: Mode,
    context: Option<String>,
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

    match args.mode {
        Mode::Namespace => {
            let contexts = commands::get_namespace();
            let namespace = commands::selectable_list(contexts);
            selection = commands::get_current_context();

            commands::set_namespace(&selection, &namespace, &temp_dir);
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
