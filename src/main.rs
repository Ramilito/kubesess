mod commands;
mod config;

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

    match args.mode {
        Mode::Namespace => {
            let ctx = commands::get_current_context();
            let ns = selection(args.value, || -> String {
                let namespaces = commands::get_namespaces();
                commands::selectable_list(namespaces)
            });

            commands::set_namespace(&ctx, &ns, &temp_dir);
        }
        Mode::Context => {
            let ctx = selection(args.value, || -> String {
                let contexts = commands::get_context();
                commands::selectable_list(contexts)
            });

            commands::set_context(&ctx, &temp_dir);

            println!("{}/{}", &temp_dir, str::replace(&ctx, ":", "_"));
        }
        Mode::DefaultContext => {
            let ctx = selection(args.value, || -> String {
                let contexts = commands::get_context();
                commands::selectable_list(contexts)
            });

            commands::set_default_cotext(&ctx);
        }
    }

    Ok(())
}

fn selection(value: Option<String>, callback: fn() -> String) -> String {
    let ctx;
    match value {
        None => {
            ctx = callback();
        }
        Some(x) => ctx = x.trim().to_string(),
    }

    ctx
}
