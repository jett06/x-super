mod consts;
mod err;
mod os;
mod pkg;
mod sudo;

use crate::consts::*;
use crate::err::*;
use crate::os::*;
use crate::pkg::*;
use crate::sudo::*;
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
    /// override the default elevation handler for the given one
    #[argh(option, short = 'e', hidden_help)]
    elevation_handler: Option<String>,
}

fn main() -> Result<()> {
    // Fix an environmental variable for `skim`, otherwise the TUI is scrambled on Termux
    // platforms.
    #[cfg(target_os = "android")]
    env::set_var("TERMINFO", "/data/data/com.termux/files/usr/share/terminfo");

    let cli: Cli = argh::from_env();
    let manager = new_package_manager()?;
    #[cfg(target_os = "linux")]
    let maybe_elevation_handler = Some(
        cli.elevation_handler
            .map_or(ElevationHandler::try_from_env(), ElevationHandler::try_from)?,
    );

    #[cfg(target_os = "android")]
    let maybe_elevation_handler = None;

    let maybe_package_list = if cli.install {
        Some(manager.available_package_list()?)
    } else if cli.remove {
        Some(manager.installed_package_list()?)
    } else {
        None
    };

    let Some(package_list) = maybe_package_list else {
        eprintln!("ERROR: No switch was passed!");
        println!("{}", *HELP_TEXT);
        process::exit(1);
    };

    let query_cmd = format!("{} {{}}", manager.package_query_cmd()?);

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
                manager.interactive_install(&selected_packages, maybe_elevation_handler)?;
            } else if cli.remove {
                manager.interactive_remove(&selected_packages, maybe_elevation_handler)?;
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

    Ok(())
}
