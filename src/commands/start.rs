use std::{collections::HashMap, process::exit};

use crate::api::server::{PostServerAction, post_server};

pub async fn start_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        start_cmd_help();
        return;
    } else if args.len() < 2 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "start")
        );
        exit(1);
    }

    // TODO: should this be sequential or...? maybe --parallel for advanced users?... lol
    let mut any_errored = false;
    for server_name in args[1..].iter() {
        match post_server(server_name.to_string(), PostServerAction::Start).await {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                any_errored = true;
            }
        }
    }
    if any_errored {
        exit(1);
    }
}

pub fn start_cmd_help() {
    println!(
        "Start an app managed by Octyne.

Usage: octynectl start [OPTIONS] [APP NAMES...]

Options:
    -h, --help               Print help information"
    );
}
