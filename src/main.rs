mod commands;

use clap::Parser;
use std::io;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
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

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let temp_dir = format!("{}/.cache/kubesess", dirs::home_dir().unwrap().display());
    let ctx;

    match args.mode {
        Mode::Namespace => {
            ctx = commands::get_current_context();
            let ns;

            match args.value {
                None => {
                    let namespaces = commands::get_namespaces();
                    ns = commands::selectable_list(namespaces);
                }
                Some(x) => ns = x,
            }
            commands::set_namespace(&ctx, &ns, &temp_dir);
        }
        Mode::Context => {
            match args.value {
                None => {
                    let contexts = commands::get_context();
                    ctx = commands::selectable_list(contexts);
                }
                Some(x) => ctx = x,
            }
            commands::set_context(&ctx, &temp_dir);

            println!("{}/{}", &temp_dir, str::replace(&ctx, ":", "_"));
        }
        Mode::DefaultContext => {
        }
    }

    Ok(())
}
