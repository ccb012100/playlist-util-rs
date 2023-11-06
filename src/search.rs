mod album;
mod db_search;
mod tsv_search;

use self::album::Album;
use crate::output::Output;
use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;

pub(crate) fn search<'a>(query: &'a SearchQuery<'a>) -> Result<SearchResults<'a>> {
    if query.verbose {
        Output::info(&format!("Searching {:#?}", query));
    }
    match query.search_type {
        SearchType::Db => {
            if query.verbose {
                Output::info("Searching DB...");
            }
            db_search::search(query).with_context(|| format!("Search failed: {:#?}", query))
        }
        SearchType::Tsv => {
            if query.verbose {
                Output::info("Searching TSV...");
            }
            tsv_search::search(query).with_context(|| format!("Search failed: {:#?}", query))
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SearchQuery<'a> {
    pub file: &'a PathBuf,
    pub include_header: bool,
    pub include_playlist_name: bool,
    pub search_term: &'a str,
    pub search_type: SearchType,
    pub sort: SortFields,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SearchResults<'a> {
    pub include_header: bool,
    pub include_playlist_name: bool,
    pub results: Vec<Album>,
    pub search_term: &'a str,
    pub sort: SortFields,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SearchType {
    Db,
    Tsv,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SortFields {
    Added,
    Album,
    Artists,
    Year,
}
