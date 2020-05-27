use crate::config::Config;
use clap::{App, Arg};

mod config;
mod execute;
mod query;
mod response;
mod service;
mod suggestion_proc;

fn main() {
    let matches = App::new(env!("PROVIDER_NAME"))
        .version("1.0")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("QUERY")
                .help("Query services")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("execute")
                .short("x")
                .long("execute")
                .value_name("ID")
                .help("Launch service with given id")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("alter_execute")
                .short("X")
                .long("alt-execute")
                .value_name("ID")
                .help("Reveal service with given id")
                .takes_value(true),
        )
        .get_matches();
    if let Some(query) = matches.value_of("query") {
        let output = std::env::var("OUTPUT").expect("Cannot get OUTPUT from env");
        let identifier = std::env::var("IDENTIFIER").expect("Cannot get IDENTIFIER from env");
        let config =
            Config::from(&std::env::var("SETTINGS").expect("Cannot get SETTINGS from env"))
                .expect("settings is invalid");
        query::query(query.trim(), &output, &identifier, config);
    } else if let Some(id) = matches.value_of("execute") {
        execute::execute(id.trim());
    } else if let Some(id) = matches.value_of("alter_execute") {
        execute::execute(id.trim());
    }
}
