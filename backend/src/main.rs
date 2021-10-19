pub mod logging;
pub mod api;

use clap::{Arg, App, AppSettings};

#[tokio::main]
async fn main() {
    logging::init();

    // We could technically take a configuration file, but I only have 1.5 hours to finish this.
    let matches = App::new("Accurate (news) API Server")
        .version("1.0,0")
        .author("Milo Banks <milobanks@rowlandhall.org>")
        .about("Servers the accurate (news) API")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(App::new("serve")
            .about("Serves the API")
            .arg(Arg::new("host")
                    .short('s')
                    .long("host")
                    .takes_value(true)
                    .about("Where to host")
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("serve", serve_args)) => {
            api::serve(serve_args).await;
        },

        _ => unreachable!(),
    };
}
