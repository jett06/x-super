mod consts;
mod distro;
mod pkg;

use crate::consts::*;
use crate::distro::*;
use crate::pkg::*;
use argh::FromArgs;
use skim::prelude::*;
use std::{
    env,
    io::Cursor,
    process,
};

/// Terminal user interface for Linux package managers
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))]
struct Cli {
    /// select packages to install
    #[argh(switch, short = 'i')]
    install: bool,
    /// select packages to remove
    #[argh(switch, short = 'r')]
    remove: bool,
}

impl Cli {
    fn help(&self) {
        let stdout = process::Command::new(env::current_exe().unwrap())
            .arg("-h")
            .output()
            .unwrap()
            .stdout;
        println!("{}", String::from_utf8_lossy(&stdout));
    }
}

fn main() {
    // Fix an environmental variable for `skim`, otherwise the TUI is scrambled on Termux
    // platforms.
    if *IS_TERMUX {
        env::set_var("TERMINFO", "/data/data/com.termux/files/usr/share/terminfo");
    }

    let cli: Cli = argh::from_env();
    let manager = PackageManager::try_from_env().unwrap();

    let maybe_package_list = if cli.install {
        manager.available_packages().ok()
    } else if cli.remove {
        manager.installed_packages().ok()
    } else {
        None
    };

    let Some(package_list) = maybe_package_list else {
        eprintln!("ERROR: No switch was passed!");
        cli.help();
        process::exit(1);
    };

    let maybe_query_cmd = manager.query_cmd();
    let query_cmd = if let Some(cmd) = maybe_query_cmd {
        format!("{} {{}}", cmd)
    } else {
        eprintln!(
            "Conversion of your package manager from an `OsStr` to a `String` object failed!"
        );
        process::exit(1);
    };

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
                manager.install(&selected_packages);
            } else if cli.remove {
                manager.remove(&selected_packages);
            } else {
                eprintln!("ERROR: No switch passed!");
                cli.help();
                process::exit(1);
            };
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
