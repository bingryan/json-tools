pub mod cli;

use std::process;
use clap::{App, Arg};

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("input", Some(matches)) => {
            println!("this is input");
        }
        _ => {
            println!("this is default")
        }
    }
}
