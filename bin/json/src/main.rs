use std::{process, io};
use std::fs;
use app::App;
use anyhow::Result;
use log;
use serde_json::Value;
use serde::{self, Serialize};

use common::logger::init_logger;
use input;
use format::json::JsonPrettyFormatter;

pub mod cli;


fn run() -> Result<bool> {
    init_logger();
    let mut no_errors: bool = true;

    let app = App::new(cli::build_cli_func)?;

    for (_index, input) in app.inputs()?.into_iter().enumerate() {
        match input.open(io::stdin().lock()) {
            Err(error) => {
                log::error!("Error: {}", error);
                no_errors = false;
            }
            Ok(mut opened_input) => {
                let input_str = match opened_input.kind {
                    input::OpenedInputKind::OrdinaryFile(ref path) => {
                        fs::read_to_string(path)?
                    }
                    input::OpenedInputKind::StdIn => {
                        let mut line_buffer = Vec::new();

                        while opened_input.reader.read_line(&mut line_buffer)? {}

                        String::from_utf8(line_buffer)?
                    }
                    _ => "{}".to_string(),
                };

                let obj: Value = serde_json::from_str(input_str.as_ref())?;
                let buf = Vec::new();
                let formatter = JsonPrettyFormatter::new();
                let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
                obj.serialize(&mut ser).unwrap();
                println!("{}", String::from_utf8(ser.into_inner()).unwrap());
            }
        }
    }

    Ok(no_errors)
}

fn main() {
    let result = run();

    match result {
        Err(error) => {
            log::error!("Json processor Error: {}", error);
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
