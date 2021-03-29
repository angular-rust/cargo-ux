use clap::{crate_authors, crate_description, crate_version, App, AppSettings, SubCommand};

pub fn create<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo ux")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        // .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAuto)
        // .setting(AppSettings::NoBinaryName)
        .bin_name("cargo ux")
        // .args(&[
        //     // Arg::from_usage("--config <FILE> 'a required file for the configuration and no short'"),
        //     // Arg::from_usage("-d, --debug... 'turns on debugging information and allows multiples'"),
        //     // Arg::from_usage("-v...                'Sets the level of verbosity'"),
        //     // Arg::from_usage("[input] 'an optional input file to use'"),
        // ])
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds support for an external library")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("analytics")
                .about("Configures the gathering of usage metrics")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Compiles an app into an output directory")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Retrieves or sets configuration values")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Invokes the deploy builder for a specified project")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("doc")
                .about("Opens the official Angular Rust documentation")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("e2e")
                .about("Runs end-to-end tests using Protractor")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("extract-i18n")
                .about("Extracts i18n messages from source code")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates files based on a schematic")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("lint")
                .about("Runs linting tools on app code")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new Angular Rust application")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs an target with an optional custom builder")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Builds and serves your app")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Runs unit tests in a project")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates your application and its dependencies")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
}
