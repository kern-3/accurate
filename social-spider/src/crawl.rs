use clap::ArgMatches;

use super::process;
use std::time;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use log::{info, error};
use futures::future;

pub struct State {
    // All the posts we have yet to process. The
    // key refers to the URL of the post.
    pub post_queue: HashSet<String>,

    // All the sources we have yet to process.
    // The key refers to the source (URL), and
    // the value refers to the post (again, URL).
    pub source_queue: HashMap<String, String>,
}

impl State {
    pub fn new(starting_url: String) -> Self {
        Self {
            post_queue: HashSet::from_iter(vec![
                starting_url,
            ]),

            source_queue: HashMap::new(),
        }
    }
}

pub async fn crawl<'a>(matches: &ArgMatches<'a>) {
    let mut epoch_time = time::Instant::now();
    let mut epoch_number: u64 = 0;
    let mut state = State::new(
        matches.value_of("start").expect("unwraped defaulted argument").to_owned()
    );

    loop {
        let mut post_handles = vec![];
        let mut source_handles = vec![];

        // Spwan the post processes for this epoch
        for url in state.post_queue.clone() {
            post_handles.push(
                tokio::spawn(async move {
                    process::post::process_post(url)
                })
            );
        }

        // Join all the state handles
        while ! post_handles.is_empty() {
            match future::select_all(post_handles).await {
                (Ok(_val), _index, remaining) => {
                    post_handles = remaining;
                },

                (Err(e), _index, remaining) => {
                    error!("Failed to join/select post future (possible runtime failure). {}", e);
                    post_handles = remaining;
                }
            }
        }

        // Spawn the source processes for this epoch
        for url in state.source_queue.clone() {
            source_handles.push(
                tokio::spawn(async move {
                    process::source::process_source(url.0)
                })
            );
        }

        // Join all the source handles. I know this joins after all
        // the state handles have been joined, but these should complete
        // at roughly the same time.
        // NOTE: I don't believe that the processes start only after being
        // NOTE: awaited, but this may be worth checking. At a later date.
        while ! source_handles.is_empty() {
            match future::select_all(source_handles).await {
                (Ok(_val), _index, remaining) => {
                    source_handles = remaining;
                },

                (Err(e), _index, remaining) => {
                    error!("Failed to join/select source future (possible runtime failure). {}", e);
                    source_handles = remaining;
                }
            }
        }

        // Informational stuff :)
        info!("Successfully completed epoch {:8} in {} seconds!", epoch_number, epoch_time.elapsed().as_secs());
        epoch_number += 1;
        epoch_time = time::Instant::now();
    }
}
