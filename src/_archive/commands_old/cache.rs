use std::time::Duration;

use anyhow::{bail, Context, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::app::Cache;
use std::fs;

#[derive(clap::Subcommand, Clone, Copy)]
pub enum Commands {
    /// Print cache root
    Path,
    /// List caches
    List {
        /// Prints entries under namespaces
        #[arg(short)]
        detailed: bool,
    },
    /// Open the cache folder
    Open,
    /// Delete everything from cache (no confirmation)
    Clear,
}

pub fn run(commands: Commands) -> Result<()> {
    match Cache::cache_root() {
        Some(cache_folder) => {
            match commands {
                Commands::Path => {
                    println!("{}", cache_folder.to_string_lossy());
                },

                Commands::List { detailed } => {
                    println!(
                        " Listing cache...\n {}",
                        style(format!("Folder: {}", cache_folder.to_string_lossy())).dim()
                    );

                    let mut namespaces = 0;
                    let mut all = 0;
                    for entry in fs::read_dir(&cache_folder)? {
                        let entry = entry?;
                        let count = fs::read_dir(entry.path())?.count();
                        println!(
                            " {} {} {} {count} entries",
                            style("=>").cyan(),
                            style(entry.file_name().to_string_lossy()).green().bold(),
                            style("-").dim()
                        );

                        if detailed {
                            for entry in fs::read_dir(entry.path())? {
                                let entry = entry?;
                                println!(
                                    "    {} {}",
                                    style("└").green(),
                                    style(entry.file_name().to_string_lossy()).dim(),
                                );
                            }
                        }

                        namespaces += 1;
                        all += count;
                    }

                    println!(" {all} entries in {namespaces} namespaces in total");
                },

                Commands::Open => {
                    opener::open(cache_folder).context("Opening cache folder")?;
                },

                Commands::Clear => {
                    let pb = ProgressBar::new_spinner()
                        .with_style(ProgressStyle::with_template(
                            "{spinner:.cyan.bold} {prefix:.green} {message}",
                        )?)
                        .with_prefix("Deleting");
                    pb.enable_steady_tick(Duration::from_millis(250));

                    for entry in fs::read_dir(&cache_folder)? {
                        let entry = entry?;

                        pb.set_message(entry.file_name().to_string_lossy().to_string());

                        fs::remove_dir_all(entry.path())?;
                    }

                    pb.finish_and_clear();
                    println!(" Cache has been cleared");
                },
            }

            Ok(())
        },

        None => bail!("Cache directory was missing, maybe it's disabled?"),
    }
}
