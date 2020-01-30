extern crate clap;
extern crate git2;
extern crate serde;
extern crate toml;

mod config;
mod utils;

use crate::config::Config;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("architect")
        .version("0.1.0")
        .about("Automated debian package building")
        .author("Hayden Hughes <hayden@firemail.cc>")
        .arg(
            Arg::with_name("config")
                .help("Specifies config file (defaults to \"architect.toml\")")
                .short("c")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("repo")
                .help("Specifies a repo name to build (defaults to building all)")
                .short("r")
                .takes_value(true),
        )
        .get_matches();

    let config: Config = toml::from_str(
        fs::read_to_string(matches.value_of("config").unwrap_or("architect.toml"))
            .expect("Unable to read config")
            .as_str(),
    )
    .expect("Unable to parse config");

    if let Some(repo) = matches.value_of("repo") {
        config.build(config.get_repo(repo).expect("Repo doesn't exist"));
    } else {
        config.build_all();
    }
}
