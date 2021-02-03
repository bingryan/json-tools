use clap::{crate_name, crate_version, App as ClapApp, AppSettings, Arg};
use std::env;

pub fn build_cli_func(interactive_output: bool) -> ClapApp<'static, 'static> {
    let clap_color_setting = if interactive_output && env::var_os("NO_COLOR").is_none() {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    ClapApp::new(crate_name!())
        .version(crate_version!())
        .global_setting(clap_color_setting)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::HidePossibleValuesInHelp)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .about(
            "json is a command-line JSON processor.",
        )
        .arg(
            Arg::with_name("FILE")
                .help("File(s) to print")
                .multiple(true)
                .empty_values(false),
        )
}
