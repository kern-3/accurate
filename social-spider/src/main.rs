pub mod logging;
pub mod crawl;
pub mod process;

use log::info;
use clap::{App, Arg, SubCommand, AppSettings};

#[tokio::main]
async fn main() {
    // First, we set up logging
    logging::apply_dispatch();

    // Create our CLI outline
    let matches = App::new("Social Spider")
        .version("1.0.0")
        .author("Milo Banks <milobanks@rowlandhall.org>")
        .about("Crawls Twitter and assembles a dataset")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("crawl")
            .about("Actually crawls Twitter")
            .arg(Arg::with_name("posts")
                 .short("p")
                 .long("post-output")
                 .default_value("posts_data.json"))
            .arg(Arg::with_name("metadata")
                 .short("m")
                 .long("metadata-output")
                 .default_value("metadata_data.json"))
            .arg(Arg::with_name("start")
                 .short("s")
                 .long("starting-url")
                 .default_value("https://twitter.com/UN/status/1448770842091995140?s=20"))
        ).get_matches();

    // Match against the subcommands
    match matches.subcommand() {
        ("crawl", Some(crawl_args)) => {
            info!("Crawling!");

            crawl::crawl(crawl_args).await;
        }

        // This should never happen (subcommand is set as required for clap)
        _ => unreachable!(),
    }
}
