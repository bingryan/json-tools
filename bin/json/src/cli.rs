use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("json")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("input")
                .help("The input value.")
                .empty_values(false),
        )
}
