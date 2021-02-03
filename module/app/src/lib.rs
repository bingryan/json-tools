use clap::{App as ClapApp, ArgMatches};
use anyhow::{Result, Error};
use std::ffi::OsStr;
use input::{new_stdin_input, new_file_input, Input};
use atty::{self, Stream};


#[allow(dead_code)]
pub struct App {
    pub matches: ArgMatches<'static>,
    interactive_output: bool,
}

impl App {
    pub fn new<F>(clap_build_func: F) -> Result<Self>
        where F: Fn(bool) -> ClapApp<'static, 'static> {
        #[cfg(windows)]
            let _ = ansi_term::enable_ansi_support();

        let interactive_output = atty::is(Stream::Stdout);

        Ok(Self {
            matches: Self::matches(clap_build_func, interactive_output)?,
            interactive_output,
        })
    }

    pub fn matches<F>(clap_build_func: F, interactive_output: bool) -> Result<ArgMatches<'static>>
        where F: Fn(bool) -> ClapApp<'static, 'static> {
        let args = wild::args_os().collect::<Vec<_>>();

        Ok(clap_build_func(interactive_output).get_matches_from(args))
    }

    pub fn inputs(&self) -> Result<Vec<Input>> {
        // verify equal length of file-names and input FILEs
        match self.matches.values_of("file-name") {
            Some(ref filenames)
            if self.matches.values_of_os("FILE").is_some()
                && filenames.len() != self.matches.values_of_os("FILE").unwrap().len() =>
                {
                    return Err(Error::msg(format!("Must be one file name per input type.")));
                }
            _ => {}
        }
        let filenames: Option<Vec<&str>> = self
            .matches
            .values_of("file-name")
            .map(|values| values.collect());

        let mut filenames_or_none: Box<dyn Iterator<Item=_>> = match filenames {
            Some(ref filenames) => Box::new(filenames.iter().map(|name| Some(OsStr::new(*name)))),
            None => Box::new(std::iter::repeat(None)),
        };
        let files: Option<Vec<&OsStr>> = self.matches.values_of_os("FILE").map(|vs| vs.collect());

        if files.is_none() {
            return Ok(vec![new_stdin_input(
                filenames_or_none.next().unwrap_or(None),
            )]);
        }
        let files_or_none: Box<dyn Iterator<Item=_>> = match files {
            Some(ref files) => Box::new(files.iter().map(|name| Some(*name))),
            None => Box::new(std::iter::repeat(None)),
        };

        let mut file_input = Vec::new();
        for (filepath, provided_name) in files_or_none.zip(filenames_or_none) {
            if let Some(filepath) = filepath {
                if filepath.to_str().unwrap_or_default() == "-" {
                    file_input.push(new_stdin_input(provided_name));
                } else {
                    file_input.push(new_file_input(filepath, provided_name));
                }
            }
        }
        Ok(file_input)
    }
}