use crate::config::Config;

mod config;
mod execute;
mod query;
mod service;
mod suggestion_proc;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let action = args.next().expect("Unable to get action");
    let content = args.next().expect("Unable to get content");
    match action.trim() {
        "-q" | "--query" => {
            let config =
                Config::from(&std::env::var("SETTINGS").expect("Cannot get SETTINGS from env"))
                    .expect("settings is invalid");
            query::query(content.trim(), config);
        }
        "-x" | "--execute" => execute::execute(content.trim()),
        "-X" | "--alter_execute" => execute::preview(content.trim()),
        _ => panic!("Unsupported action flag"),
    }
}
