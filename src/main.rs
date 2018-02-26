#[macro_use]
extern crate quicli;
use quicli::prelude::*;

extern crate plist;
extern crate serde;
extern crate serde_json;
extern crate serde_transcode;

use std::fs::File;
use std::path::PathBuf;
use serde_transcode::transcode;

/// Converts the specified plist file to JSON
#[derive(StructOpt)]
struct Cli {
    /// Plist file
    #[structopt(parse(from_os_str))]
    source: PathBuf,

    /// JSON filename. Output is written to stdout if not specified.
    #[structopt(parse(from_os_str))]
    target: Option<PathBuf>,

    /// Output human-readable JSON
    #[structopt(long="pretty")]
    pretty: bool,

    /// Add verbosity to output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

main!{|args: Cli, log_level: verbosity| {
    let plist_reader = File::open(args.source)?;
    // debug!("{:#?}", plist::Plist::read(plist_reader)?);
    let mut plist_deserializer = plist::serde::Deserializer::from_reader(plist_reader);

    let stdout = std::io::stdout();

    match (args.target, args.pretty) {
        (None, false) => transcode(&mut plist_deserializer, &mut serde_json::Serializer::new(stdout.lock())),
        (None, true) => transcode(&mut plist_deserializer, &mut serde_json::Serializer::pretty(stdout.lock())),
        (Some(target), false) => transcode(&mut plist_deserializer, &mut serde_json::Serializer::new(File::create(target)?)),
        (Some(target), true) => transcode(&mut plist_deserializer, &mut serde_json::Serializer::pretty(File::create(target)?)),
    }?;
}}
