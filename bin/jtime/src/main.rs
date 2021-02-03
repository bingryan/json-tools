#[macro_use]
extern crate serde_json;

use chrono::{Utc, DateTime};
use format::json::JsonPrettyFormatter;
use chrono::prelude::*;
use serde::Serialize;
use common::logger::init_logger;
use clap::{App, AppSettings, crate_name, crate_version, crate_authors, crate_description, Arg};

use anyhow::Result;
use std::process;


fn run() -> Result<bool> {
    init_logger();
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::HidePossibleValuesInHelp)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .about(
            "json time is a command-line time of human command.",
        )
        .args(&[Arg::with_name("timestamp")
            .short("t")
            .long("timestamp")
            .multiple(true)
            .empty_values(false)
            .help("format timestamp ,for example: -t 1612364421"), ])
        .get_matches();

    let time: DateTime<Utc> = match matches.value_of("timestamp") {
        Some(timestamp) => {
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp.parse()?, 0), Utc)
        }
        _ => {
            Utc::now()
        }
    };
    let obj = json!({"UTC": time.to_string(),
                            "timestamp": time.timestamp().to_string().parse::<i32>().unwrap(),
                            "RFC 2822": time.to_rfc2822().to_string(),
                            "RFC 3339": time.to_rfc3339().to_string(),
                            "local": time.with_timezone(&Local).to_string()});

    let buf = Vec::new();
    let formatter = JsonPrettyFormatter::new();

    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser).unwrap();
    println!("{}", String::from_utf8(ser.into_inner()).unwrap());
    Ok(true)
}

fn main() {
    let result = run();
    match result {
        Err(error) => {
            log::error!("Json time Error: {}", error);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}