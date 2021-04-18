use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg};

pub fn create<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAuto)
        .bin_name("cargo ux")
        .subcommand(add())
        .subcommand(analytics())
        .subcommand(build())
        .subcommand(config())
        .subcommand(deploy())
        .subcommand(doc())
        .subcommand(e2e())
        .subcommand(i18n())
        .subcommand(generate())
        .subcommand(lint())
        .subcommand(new())
        .subcommand(run())
        .subcommand(serve())
        .subcommand(test())
        .subcommand(update())
}

fn add<'a, 'b>() -> App<'a, 'b> {
    App::new("add")
        .about("Adds support for an external library")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("collection")
                .required(true)
                .help("The package to be added [string]"),
        )
        .arg(
            Arg::with_name("defaults")
                .long("defaults")
                .help("Disable interactive input prompts for options with a default"),
        )
        .arg(
            Arg::with_name("interactive")
                .long("interactive")
                .help("Enable interactive input prompts"),
        )
        .arg(
            Arg::with_name("registry")
                .long("registry")
                .help("The registry to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Display additional details about internal operations during execution"),
        )
}

fn analytics<'a, 'b>() -> App<'a, 'b> {
    App::new("analytics")
        .about("Configures the gathering of usage metrics")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("setting-or-project")
                .help("Directly enables or disables all usage analytics for the user"),
        )
        .arg(Arg::with_name("project-setting").help("Sets the default analytics enablement status"))
}

fn build<'a, 'b>() -> App<'a, 'b> {
    App::new("build")
        .about("Compiles an app into an output directory")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to build")
        )
        .arg(
            Arg::with_name("allowed-common-js-dependencies")
                .long("allowed-common-js-dependencies")
                .help("A list of CommonJS packages that are allowed to be used without a build time warning")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("aot")
                .long("aot")
                .help("Build using Ahead of Time compilation")
        )
        .arg(
            Arg::with_name("base-href")
                .long("base-href")
                .help("Base url for the application being built")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("build-optimizer")
                .long("build-optimizer")
                .help("Enables '@angular-devkit/build-optimizer' optimizations when using the 'aot' option")
        )
        .arg(
            Arg::with_name("common-chunk")
                .long("common-chunk")
                .help("Generate a seperate bundle containing code used across multiple bundles")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("One or more named builder configurations as a comma-separated list as specified in the 'configurations' section of ux.yaml")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("cross-origin")
                .long("cross-origin")
                .help("Define the crossorigin attribute setting of elements that provide CORS support")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("delete-output-path")
                .long("delete-output-path")
                .help("Delete the output path before building")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("deploy-url")
                .long("deploy-url")
                .help("URL where files will be deployed")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("extract-licenses")
                .long("extract-licenses")
                .help("Extract all licenses in a separate file")
        )
        .arg(
            Arg::with_name("fork-type-checker")
                .long("fork-type-checker")
                .help("Run the type checker in a forked process")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("i18n-missing-translation")
                .long("i18n-missing-translation")
                .help("How to handle missing translations for i18n")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("index")
                .long("index")
                .help("Configures the generation of the application's HTML index")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("localize")
                .long("localize")
                .help("Translate the bundles in one or more locales")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("main")
                .long("main")
                .help("The full path for the main entry point to the app")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("named-chunks")
                .long("named-chunks")
                .help("Use file name for lazy loaded chunks")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("optimization")
                .long("optimization")
                .help("Enables optimization of the build output")
        )
        .arg(
            Arg::with_name("output-hashing")
                .long("output-hashing")
                .help("Define the output filename cache-busting hashing mode")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output-path")
                .long("output-path")
                .help("The full path for the new output directory, relative to the current workspace")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("poll")
                .long("poll")
                .help("Enable and define the file watching poll time period in milliseconds")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("registry")
                .long("registry")
                .help("The registry to use")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("polyfills")
                .long("polyfills")
                .help("The full path for the polyfills file, relative to the current workspace")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("preserve-symlinks")
                .long("preserve-symlinks")
                .help("Do not use the real path when resolving modules")
        )
        .arg(
            Arg::with_name("prod")
                .long("prod")
                .help("Shorthand for '--configuration=production'")
        )
        .arg(
            Arg::with_name("progress")
                .long("progress")
                .help("Log progress to the console while building")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("resources-output-path")
                .long("resources-output-path")
                .help("The path where style resources will be placed, relative to outputPath")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("service-worker")
                .long("service-worker")
                .help("Generates a service worker config for production builds")
        )
        .arg(
            Arg::with_name("show-circular-dependencies")
                .long("show-circular-dependencies")
                .help("Show circular dependency warnings on builds")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("source-map")
                .long("source-map")
                .help("Output source maps for scripts and styles")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("stats-json")
                .long("stats-json")
                .help("Generates a 'stats.json' file which can be analyzed later")
        )
        .arg(
            Arg::with_name("subresource-integrity")
                .long("subresource-integrity")
                .help("Enables the use of subresource integrity validation")
        )
        .arg(
            Arg::with_name("vendor-chunk")
                .long("vendor-chunk")
                .help("Generate a seperate bundle containing only vendor libraries")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .help("Adds more details to output logging")
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .help("Run build when files change")
        )
}

fn config<'a, 'b>() -> App<'a, 'b> {
    App::new("config")
        .about("Retrieves or sets configuration values")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("json-path")
                .help("The configuration key to set or query, in JSON path format"),
        )
        .arg(
            Arg::with_name("value")
                .help("If provided, a new value for the given configuration key"),
        )
        .arg(
            Arg::with_name("global")
                .long("global")
                .help("Access the global configuration in the caller's home directory"),
        )
}

fn deploy<'a, 'b>() -> App<'a, 'b> {
    App::new("deploy")
        .about("Invokes the deploy builder for a specified project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to deploy")
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("One or more named builder configurations as a comma-separated list as specified in the 'configurations' section of ux.yaml")
                .takes_value(true)
        )
}

fn doc<'a, 'b>() -> App<'a, 'b> {
    App::new("doc")
        .about("Opens the official Angular Rust documentation")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("keyword")
                .help("The keyword to search for, as provided in the search bar in angular-rust.github.io")
        )
        .arg(
            Arg::with_name("search")
                .long("search")
                .help("Search all of angular-rust.github.io. Otherwise, searches only API reference documentation")
        )
        .arg(
            Arg::with_name("version")
                .long("version")
                .help("Contains the version of Angular Rust to use for the documentation")
                .takes_value(true)
        )
}

fn e2e<'a, 'b>() -> App<'a, 'b> {
    App::new("e2e")
        .about("Runs end-to-end tests using Protractor")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to build")
        )
        .arg(
            Arg::with_name("base-url")
                .long("base-url")
                .help("Base URL for protractor to connect to")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("One or more named builder configurations as a comma-separated list as specified in the 'configurations' section of ux.yaml")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("dev-server-target")
                .long("dev-server-target")
                .help("A dev-server builder target to run tests against in the format of project:target[:configuration]")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("grep")
                .long("grep")
                .help("Execute specs whose names match the pattern, which is internally compiled to a RegExp")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .help("Host to listen on")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("invert-grep")
                .long("invert-grep")
                .help("Invert the selection specified by the 'grep' option")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .help("The port to use to serve the application")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prod")
                .long("prod")
                .help("Shorthand for '--configuration=production'")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("protractor-config")
                .long("protractor-config")
                .help("The name of the Protractor configuration file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("specs")
                .long("specs")
                .help("Override specs in the protractor config")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("suite")
                .long("suite")
                .help("Override suite in the protractor config")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("webdriver-update")
                .long("webdriver-update")
                .help("Try to update webdriver")
                .takes_value(true)
        )
}

fn i18n<'a, 'b>() -> App<'a, 'b> {
    App::new("i18n")
        .about("Extracts i18n messages from source code")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to build")
        )
        .arg(
            Arg::with_name("browser-target")
                .long("browser-target")
                .help("A browser builder target to extract i18n messages in the format of project:target[:configuration]")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("One or more named builder configurations as a comma-separated list as specified in the 'configurations' section of ux.yaml")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .help("Output format for the generated file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("ivy")
                .long("ivy")
                .help("Use Ivy compiler to extract translations")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("out-file")
                .long("out-file")
                .help("Name of the file to output")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output-path")
                .long("output-path")
                .help("Path where output will be placed")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("base-url")
                .long("base-url")
                .help("Base URL for protractor to connect to")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prod")
                .long("prod")
                .help("Shorthand for '--configuration=production'.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("progress")
                .long("progress")
                .help("Log progress to the console")
                .takes_value(true)
        )
}

fn generate<'a, 'b>() -> App<'a, 'b> {
    App::new("generate")
        .about("Generates files based on a schematic")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .subcommand(generate_appshell())
        .subcommand(generate_application())
        .subcommand(generate_class())
        .subcommand(generate_component())
        .subcommand(generate_directive())
        .subcommand(generate_enum())
        .subcommand(generate_guard())
        .subcommand(generate_interceptor())
        .subcommand(generate_interface())
        .subcommand(generate_library())
        .subcommand(generate_module())
        .subcommand(generate_pipe())
        .subcommand(generate_resolver())
        .subcommand(generate_service())
        .subcommand(generate_serviceworker())
        .subcommand(generate_webworker())
        .arg(
            Arg::with_name("defaults")
                .long("defaults")
                .help("Disable interactive input prompts for options with a default")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .help("Run through and reports activity without writing out results")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("Force overwriting of existing files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interactive")
                .long("interactive")
                .help("Enable interactive input prompts")
                .takes_value(true),
        )
}

fn generate_appshell<'a, 'b>() -> App<'a, 'b> {
    App::new("appshell")
        .about("Generates an app shell for running a server-side version of an app")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        // .arg(
        //     Arg::with_name("project")
        //         .help("The name of the project to build")
        // )
        .arg(
            Arg::with_name("app-dir")
                .long("app-dir")
                .help("The name of the application directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("app-id")
                .long("app-id")
                .help("The app ID to use in withServerTransition()")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client-project")
                .long("client-project")
                .help("The name of the related client app")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("main")
                .long("main")
                .help("The name of the main entry-point file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("root-module-class-name")
                .long("root-module-class-name")
                .help("The name of the root module class")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("root-module-file-name")
                .long("root-module-file-name")
                .help("The name of the root module file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("route")
                .long("route")
                .help("Route path used to produce the app shell")
                .takes_value(true),
        )
}

fn generate_application<'a, 'b>() -> App<'a, 'b> {
    App::new("application")
        .about("Generates a new basic app definition in the 'projects' subfolder of the workspace")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("name")
                .help("The name of the new app")
        )
        .arg(
            Arg::with_name("inline-style")
                .long("inline-style")
                .help("Include styles inline in the root component.rs file.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("inline-template")
                .long("inline-template")
                .help("Include template inline in the root component.rs file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("legacy-browsers")
                .long("legacy-browsers")
                .help("Add support for legacy browsers like Internet Explorer using differential loading")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("minimal")
                .long("minimal")
                .help("Create a bare-bones project without any testing frameworks")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .help("A prefix to apply to generated selectors")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("routing")
                .long("routing")
                .help("Create a routing module")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-install")
                .long("skip-install")
                .help("Skip installing dependency packages")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-package-yaml")
                .long("skip-package-yaml")
                .help("Do not add dependencies to the 'package.yaml' file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the application")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("strict")
                .long("strict")
                .help("Creates an application with stricter bundle budgets settings")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("view-encapsulation")
                .long("view-encapsulation")
                .help("The view encapsulation strategy to use in the new app")
                .takes_value(true)
        )
}

fn generate_class<'a, 'b>() -> App<'a, 'b> {
    App::new("class")
        .about("Creates a new generic class definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the new class"))
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new class")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("type")
                .long("type")
                .help("Adds a developer-defined type to the filename, in the format 'name.type.rs'")
                .takes_value(true),
        )
}

fn generate_component<'a, 'b>() -> App<'a, 'b> {
    App::new("component")
        .about("Creates a new generic component definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the component"))
        .arg(
            Arg::with_name("change-detection")
                .long("change-detection")
                .help("The change detection strategy to use in the new component")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("display-block")
                .long("display-block")
                .help("Specifies if the style will contain :host { display: block; }")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("export")
                .long("export")
                .help("The declaring module exports this component")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("Create the new files at the top level of the current project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inline-style")
                .long("inline-style")
                .help("Include styles inline in the component.rs file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inline-template")
                .long("inline-template")
                .help("Include template inline in the component.rs file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("module")
                .long("module")
                .help("The declaring module")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .help("The prefix to apply to the generated component selector")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("collection")
                .long("collection")
                .help("Abrowserbuildertargettoextract")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("selector")
                .long("selector")
                .help("The HTML selector to use for this component")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-import")
                .long("skip-import")
                .help("Do not import this component into the owning module")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-selector")
                .long("skip-selector")
                .help("Specifies if the component should have a selector or not")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new component")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("type")
                .long("type")
                .help("Adds a developer-defined type to the filename, in the format 'name.type.rs'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("view-encapsulation")
                .long("view-encapsulation")
                .help("The view encapsulation strategy to use in the new component")
                .takes_value(true),
        )
}

fn generate_directive<'a, 'b>() -> App<'a, 'b> {
    App::new("directive")
        .about("Creates a new generic directive definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("name")
                .help("The name of the new directive")
        )
        .arg(
            Arg::with_name("export")
                .long("export")
                .help("The declaring module exports this directive")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("When true (the default), creates the new files at the top level of the current project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("module")
                .long("module")
                .help("The declaring module")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .help("A prefix to apply to generated selectors")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("selector")
                .long("selector")
                .help("The HTML selector to use for this directive")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-import")
                .long("skip-import")
                .help("Do not import this directive into the owning module")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new class")
                .takes_value(true)
        )
}

fn generate_enum<'a, 'b>() -> App<'a, 'b> {
    App::new("enum")
        .about("Generates a new, generic enum definition for the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the enum"))
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project in which to create the enum")
                .takes_value(true),
        )
}

fn generate_guard<'a, 'b>() -> App<'a, 'b> {
    App::new("guard")
        .about("Generates a new, generic route guard definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("name")
                .help("The name of the new route guard")
        )
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("When true (the default), creates the new files at the top level of the current project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("implements")
                .long("implements")
                .help("Specifies which interfaces to implement")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new guard")
                .takes_value(true)
        )
}

fn generate_interceptor<'a, 'b>() -> App<'a, 'b> {
    App::new("interceptor")
        .about("Creates a new, generic interceptor definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the interceptor"))
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("When true (the default), creates files at the top level of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new interceptor")
                .takes_value(true),
        )
}

fn generate_interface<'a, 'b>() -> App<'a, 'b> {
    App::new("interface")
        .about("Creates a new generic interface definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the interface"))
        .arg(
            Arg::with_name("type").help(
                "Adds a developer-defined type to the filename, in the format 'name.type.rs'",
            ),
        )
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .help("A prefix to apply to generated selectors")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
}

fn generate_library<'a, 'b>() -> App<'a, 'b> {
    App::new("library")
        .about("Creates a new generic library project in the current workspace")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("name")
                .help("The name of the library")
        )
        .arg(
            Arg::with_name("entry-file")
                .long("entry-file")
                .help("The path at which to create the library's public API file, relative to the workspace root")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .help("A prefix to apply to generated selectors")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-install")
                .long("skip-install")
                .help("Do not install dependency packages")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-package-yaml")
                .long("skip-package-yaml")
                .help("Do not add dependencies to the 'package.yaml' file")
                .takes_value(true)
        )
}

fn generate_module<'a, 'b>() -> App<'a, 'b> {
    App::new("module")
        .about("Creates a new generic module definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the module"))
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("Create the new files at the top level of the current project root")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("module")
                .long("module")
                .help("The declaring module")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("route")
                .long("route")
                .help("The route path for a lazy-loaded module")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("routing")
                .long("routing")
                .help("Create a routing module")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("routing-scope")
                .long("routing-scope")
                .help("The scope for the new routing module")
                .takes_value(true),
        )
}

fn generate_pipe<'a, 'b>() -> App<'a, 'b> {
    App::new("pipe")
        .about("Creates a new generic pipe definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the pipe"))
        .arg(
            Arg::with_name("export")
                .long("export")
                .help("The declaring module exports this pipe")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("When true (the default) creates files at the top level of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("module")
                .long("module")
                .help("The declaring module")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-import")
                .long("skip-import")
                .help("Do not import this pipe into the owning module")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new pipe")
                .takes_value(true),
        )
}

fn generate_resolver<'a, 'b>() -> App<'a, 'b> {
    App::new("resolver")
        .about("Generates a new, generic resolver definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("name")
                .help("The name of the new resolver")
        )
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("When true (the default), creates the new files at the top level of the current project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new resolver")
                .takes_value(true)
        )
}

fn generate_service<'a, 'b>() -> App<'a, 'b> {
    App::new("service")
        .about("Creates a new, generic service definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the service"))
        .arg(
            Arg::with_name("flat")
                .long("flat")
                .help("When true (the default), creates files at the top level of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not create 'spec.rs' test files for the new service")
                .takes_value(true),
        )
}

fn generate_serviceworker<'a, 'b>() -> App<'a, 'b> {
    App::new("serviceworker")
        .about("Pass this schematic to the 'run' command to create a service worker")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        // .arg(
        //     Arg::with_name("project")
        //         .help("The name of the project to build")
        // )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("The configuration to apply service worker to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target")
                .long("target")
                .help("The target to apply service worker to")
                .takes_value(true),
        )
}

fn generate_webworker<'a, 'b>() -> App<'a, 'b> {
    App::new("webworker")
        .about("Creates a new generic web worker definition in the given or default project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(Arg::with_name("name").help("The name of the worker"))
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The name of the project")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("snippet")
                .long("snippet")
                .help("Add a worker creation snippet in a sibling file of the same name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target")
                .long("target")
                .help("The target to apply web worker to")
                .takes_value(true),
        )
}

fn lint<'a, 'b>() -> App<'a, 'b> {
    App::new("lint")
        .about("Runs linting tools on app code")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to lint")
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("The linting configuration to use")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("exclude")
                .long("exclude")
                .help("Files to exclude from linting")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("files")
                .long("files")
                .help("Files to include in linting")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("fix")
                .long("fix")
                .help("Fixes linting errors (may overwrite linted files)")
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("Succeeds even if there was linting errors")
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .help("Output format (prose, json, stylish, verbose, pmd, msbuild, checkstyle, vso, fileslist)")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("silent")
                .long("silent")
                .help("Show output text")
        )
        .arg(
            Arg::with_name("type-check")
                .long("type-check")
                .help("Controls the type check for linting")
        )
}

fn new<'a, 'b>() -> App<'a, 'b> {
    App::new("new")
        .about("Creates a new Angular Rust application")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to create")
        )
        .arg(
            Arg::with_name("collection")
                .long("collection")
                .help("A collection of schematics to use in generating the initial application")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("commit")
                .long("commit")
                .help("Initial git repository commit information")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("create-application")
                .long("create-application")
                .help("Create a new initial application project in the 'src' folder of the new workspace")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("defaults")
                .long("defaults")
                .help("Disable interactive input prompts for options with a default")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("directory")
                .long("directory")
                .help("The directory name to create the workspace in")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .help("Run through and reports activity without writing out results")
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("Force overwriting of existing files")
        )
        .arg(
            Arg::with_name("inline-style")
                .long("inline-style")
                .help("Include styles inline in the component file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("inline-template")
                .long("inline-template")
                .help("Include template inline in the component file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("interactive")
                .long("interactive")
                .help("Enable interactive input prompts")
        )
        .arg(
            Arg::with_name("legacy-browsers")
                .long("legacy-browsers")
                .help("Add support for legacy browsers like Internet Explorer using differential loading")
        )
        .arg(
            Arg::with_name("minimal")
                .long("minimal")
                .help("Create a workspace without any testing frameworks")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("new-project-root")
                .long("new-project-root")
                .help("The path where new projects will be created, relative to the new workspace root")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .help("The prefix to apply to generated selectors for the initial project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("routing")
                .long("routing")
                .help("Generate a routing module for the initial project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-git")
                .long("skip-git")
                .help("Do not initialize a git repository")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-install")
                .long("skip-install")
                .help("Do not install dependency packages")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("skip-tests")
                .long("skip-tests")
                .help("Do not generate 'spec.rs' test files for the new project")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("strict")
                .long("strict")
                .help("Creates a workspace with stricter type checking and stricter bundle budgets settings")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .help("Add more details to output logging")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("view-encapsulation")
                .long("view-encapsulation")
                .help("The view encapsulation strategy to use in the initial project")
                .takes_value(true)
        )
}

fn run<'a, 'b>() -> App<'a, 'b> {
    App::new("run")
        .about("Runs an target with an optional custom builder")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The Architect target to run in format 'project:target[:configuration]'")
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("One or more named builder configurations as a comma-separated list as specified in the 'configurations' section of ux.yaml")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("collection")
                .long("collection")
                .help("Abrowserbuildertargettoextract")
                .takes_value(true)
        )
}

fn serve<'a, 'b>() -> App<'a, 'b> {
    App::new("serve")
        .about("Builds and serves your app")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to build")
        )
        .arg(
            Arg::with_name("browser-target")
                .long("browser-target")
                .help("A browser builder target to serve in the format of project:target[:configuration]")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("One or more named builder configurations as a comma-separated list as specified in the 'configurations' section of ux.yaml")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("disable-host-check")
                .long("disable-host-check")
                .help("Don't verify connected clients are part of allowed hosts")
        )
        .arg(
            Arg::with_name("hmr")
                .long("hmr")
                .help("Enable hot module replacement")
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .help("Host to listen on")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("live-reload")
                .long("live-reload")
                .help("Whether to reload the page on change, using live-reload")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("open")
                .long("open")
                .help("Opens the url in default browser")
        )
        .arg(
            Arg::with_name("poll")
                .long("poll")
                .help("Enable and define the file watching poll time period in milliseconds")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .help("Port to listen on")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prod")
                .long("prod")
                .help("Shorthand for '--configuration=production'")
        )
        .arg(
            Arg::with_name("proxy-config")
                .long("proxy-config")
                .help("Proxy configuration file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("public-host")
                .long("public-host")
                .help("The URL that the browser client (or live-reload client, if enabled) should use to connect to the development server")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("serve-path")
                .long("serve-path")
                .help("The pathname where the app will be served")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("ssl")
                .long("ssl")
                .help("Serve using HTTPS")
        )
        .arg(
            Arg::with_name("ssl-cert")
                .long("ssl-cert")
                .help("SSL certificate to use for serving HTTPS")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("ssl-key")
                .long("ssl-key")
                .help("SSL key to use for serving HTTPS")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .help("Adds more details to output logging")
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .help("Rebuild on change")
                .takes_value(true)
        )
}

fn test<'a, 'b>() -> App<'a, 'b> {
    App::new("test")
        .about("Runs unit tests in a project")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("project")
                .help("The name of the project to build")
        )
        .arg(
            Arg::with_name("browsers")
                .long("browsers")
                .help("Override which browsers tests are run against")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("code-coverage")
                .long("code-coverage")
                .help("Output a code coverage report")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("code-coverage-exclude")
                .long("code-coverage-exclude")
                .help("Globs to exclude from code coverage")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("configuration")
                .long("configuration")
                .help("One or more named builder configurations as a comma-separated list as specified in the 'configurations' section of ux.yaml")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("include")
                .long("include")
                .help("Globs of files to include, relative to workspace or project root")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("karma-config")
                .long("karma-config")
                .help("The name of the Karma configuration file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("main")
                .long("main")
                .help("The name of the main entry-point file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("poll")
                .long("poll")
                .help("Enable and define the file watching poll time period in milliseconds")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("polyfills")
                .long("polyfills")
                .help("The name of the polyfills file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("preserve-symlinks")
                .long("preserve-symlinks")
                .help("Do not use the real path when resolving modules")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("prod")
                .long("prod")
                .help("Shorthand for '--configuration=production'")
        )
        .arg(
            Arg::with_name("progress")
                .long("progress")
                .help("Log progress to the console while building")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("reporters")
                .long("reporters")
                .help("Karma reporters to use")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("source-map")
                .long("source-map")
                .help("Output source maps for scripts and styles")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .help("Run build when files change")
                .takes_value(true)
        )
}

fn update<'a, 'b>() -> App<'a, 'b> {
    App::new("update")
        .about("Updates your application and its dependencies")
        .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
        .arg(
            Arg::with_name("allow-dirty")
                .long("allow-dirty")
                .help("Whether to allow updating when the repository contains modified or untracked files")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("create-commits")
                .long("create-commits")
                .help("Create source control commits for updates and migrations")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("If false, will error out if installed packages are incompatible with the update")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("from")
                .long("from")
                .help("Version from which to migrate from")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("migrate-only")
                .long("migrate-only")
                .help("Only perform a migration, do not update the installed version")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("next")
                .long("next")
                .help("Use the prerelease version, including beta and RCs")
        )
        .arg(
            Arg::with_name("packages")
                .long("packages")
                .help("The names of package(s) to update")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("to")
                .long("to")
                .help("Version up to which to apply migrations")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .help("Display additional details about internal operations during execution")
                .takes_value(true)
        )
}
