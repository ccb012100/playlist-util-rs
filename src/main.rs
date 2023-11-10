use crate::{output::Output, search::SearchType};
use anyhow::Error;
use clap::Parser;
use cli::{get_path, Cli, Commands};
use log::{debug, info, LevelFilter};
use search::SearchQuery;
use std::{path::PathBuf, process::ExitCode};

mod cli;
mod output;
mod search;

fn main() -> core::result::Result<ExitCode, Error> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        LevelFilter::Debug
    } else if cli.v {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };

    env_logger::Builder::new().filter_level(log_level).init();

    debug!("logging initialized");
    debug!("parsed Cli: {:#?}", &cli);

    let path: PathBuf = get_path(&cli.file_name, cli.file_type)?;

    match &cli.command {
        Commands::Search {
            include_header,
            include_playlist_name,
            no_format,
            sort,
            term,
        } => {
            info!("Searching...");

            let query: SearchQuery = SearchQuery {
                search_term: &term.join(" "),
                search_type: match &cli.file_type {
                    cli::FileType::Db => SearchType::Db,
                    cli::FileType::Tsv => SearchType::Tsv,
                },
                file: &path,
                include_header: *include_header,
                include_playlist_name: *include_playlist_name,
                sort: search::SortFields::from(*sort),
            };

            let results: search::SearchResults<'_> = search::search(&query)?;

            match no_format {
                true => Output::search_results(&results),
                false => Output::search_results_table(&results),
            }
        }
        Commands::Sync {} => {
            info!("Syncing...");
            todo!()
        }
    }

    info!("Done!");
    Ok(ExitCode::SUCCESS)
}
