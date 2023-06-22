pub const USAGE: &str = "{0}, run `octynectl help{1}` for more information.";

pub const INCORRECT_USAGE: &str = "Incorrect usage";

const UNKNOWN_SUBCOMMAND: &str = "Unknown subcommand";

pub fn unknown_subcommand(subcommand: &str) -> String {
    if subcommand.is_empty() {
        return UNKNOWN_SUBCOMMAND.to_string();
    }
    format!("{}: {}", UNKNOWN_SUBCOMMAND, subcommand)
}

pub fn invalid_usage(msg: &str, subcommand: &str) -> String {
    if subcommand.is_empty() {
        return USAGE.replace("{0}", msg).replace("{1}", "");
    }
    USAGE
        .replace("{0}", msg)
        .replace("{1}", (" ".to_owned() + subcommand).as_str())
}

// TODO: eventually have `nodes` and --node=NAME
pub const HELP_STR: &str = "Command-line interface to control Octyne.
This connects to your local Octyne instance over Unix socket, and lets you view
and control applications running under it.

Usage: octynectl [OPTIONS] [SUBCOMMAND]

Options:
    -v, --version            Print version info and exit
    -h, --help               Print help information

Subcommands:
    list, list-apps, apps    List all apps under Octyne
    start                    Start an app managed by Octyne
    stop                     Gracefully stop an app managed by Octyne
    kill                     Kill an app managed by Octyne
    restart                  Restart an app (NOT YET IMPLEMENTED)
    status                   Get the status of an app (NOT YET IMPLEMENTED)
    logs                     Get the logs of an app (NOT YET IMPLEMENTED)
    console                  Open app console (NOT YET IMPLEMENTED)
    help                     Print this help message and exit
";