use clap::{crate_version, App, AppSettings, Arg, SubCommand};

pub fn build_app() -> App<'static, 'static> {
    let clap_color_setting = if std::env::var_os("NO_COLOR").is_none() {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let mut app = App::new("pbgopy")
        .version(crate_version!())
        .usage("pbgopy [command] [flags]")
        .setting(clap_color_setting)
        .setting(AppSettings::DeriveDisplayOrder)
        .after_help(
            "Note: `pbgopy -h` prints a short and concise overview while `pbgopy --help` gives all \
                 details.",
        );

    // Copy command
    app = app.subcommand(SubCommand::with_name("copy")
            .about("Copy from stdin")
            .arg(Arg::with_name("password")
                .help("Password to derive the symmetric-key to be used for encryption")
                .short("p")
                .long("password")
            )
        );

    app
}
