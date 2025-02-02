mod consts;
mod os;
mod pkg;

use crate::consts::*;
use crate::os::*;
use crate::pkg::*;
use argh::FromArgs;
use skim::prelude::*;
#[cfg(target_os = "android")]
use std::env;
use std::{
    io::Cursor,
    process,
};

/// Terminal user interface for Linux package managers
#[derive(FromArgs, Debug)]
#[argh(help_triggers("-h", "--help"))]
struct Cli {
    /// select packages to install
    #[argh(switch, short = 'i')]
    install: bool,
    /// select packages to remove
    #[argh(switch, short = 'r')]
    remove: bool,
}

fn main() {
    // Fix an environmental variable for `skim`, otherwise the TUI is scrambled on Termux
    // platforms.
    #[cfg(target_os = "android")]
    env::set_var("TERMINFO", "/data/data/com.termux/files/usr/share/terminfo");

    let cli: Cli = argh::from_env();
    let Some(manager) = new_package_manager() else {
        eprintln!("ERROR: Couldn't detect your package manager! Is this a supported OS?");
        process::exit(1)
    };

    let maybe_package_list = if cli.install {
        manager.available_package_list().ok()
    } else if cli.remove {
        manager.installed_package_list().ok()
    } else {
        None
    };

    let Some(package_list) = maybe_package_list else {
        eprintln!("ERROR: No switch was passed!");
        println!("{}", *HELP_TEXT);
        process::exit(1);
    };

    let query_cmd = format!("{} {{}}", manager.package_query_cmd());

    match SkimOptionsBuilder::default()
        .multi(true)
        .preview_window(Some(SKIM_PREVIEW_WINDOW))
        .preview(Some(&query_cmd))
        .build()
    {
        Ok(skim_options) => {
            let item_reader = SkimItemReader::default();
            let (items, _) = item_reader.of_bufread(Box::new(Cursor::new(package_list.join("\n"))));

            let user_selections = Skim::run_with(&skim_options, Some(items))
                .map(|output| {
                    if output.is_abort {
                        process::exit(0)
                    } else {
                        output.selected_items
                    }
                })
                .unwrap_or_default();

            let selected_packages: Vec<String> = user_selections
                .into_iter()
                .map(|sel| sel.output().to_string())
                .collect();

            if cli.install {
                manager.interactive_install(&selected_packages);
            } else if cli.remove {
                manager.interactive_remove(&selected_packages);
            }
        }
        Err(e) => {
            eprintln!(
                "Failed to initialize [`SkimOptions`] from builder! Error: {:#?}",
                e
            );
            process::exit(1);
        }
    };
}
