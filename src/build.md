# cargo ux build

> This is a Draft Version

Compiles an Angular Rust app into an output directory named dist/ at the given output path. Must be executed from within a workspace directory.

```      
cargo ux build <project> [options]
```
```   
cargo ux b <project> [options]
```

## Description

The command can be used to build a project of type "application" or "library". When used to build a library, a different builder is invoked, configuration, and watch options are applied. All other options apply only to building applications.

The application builder uses the Ninja build tool, with default configuration options specified in the workspace configuration file (ux.yaml) or with a named alternative configuration. A "production" configuration is created by default when you use the CLI to create the project, and you can use that configuration by specifying the --configuration="production" or the --prod option.

The configuration options generally correspond to the command options. You can override individual configuration defaults by specifying the corresponding options on the command line. The command can accept option names given in either dash-case or camelCase. Note that in the configuration file, you must specify names in camelCase.

Some additional options can only be set through the configuration file, either by direct editing or with the ng config command. These include assets, styles, and scripts objects that provide runtime-global resources to include in the project. Resources in CSS, such as images and fonts, are automatically written and fingerprinted at the root of the output folder.

For further details, see Workspace Configuration.

## Arguments
Argument   | Description | Value Type
-----------|-------------|-----------
project    | The name of the project to build. Can be an application or a library. | string

## Options

Option | Description | Value Type | Default Value
-------|-------------|------------|---------------
--allowed-common-js-dependencies | A list of CommonJS packages that are allowed to be used without a build time warning. | array 	
--aot  | Build using Ahead of Time compilation. | boolean | false
--base-href | Base url for the application being built. | string |
--build-optimizer | Enables '@angular-devkit/build-optimizer' optimizations when using the 'aot' option. | boolean | false
--common-chunk | Generate a seperate bundle containing code used across multiple bundles. |	boolean | true
--configuration |One or more named builder configurations as a comma-separated list as specified in the "configurations" section of ux.yaml. The builder uses the named configurations to run the given target. For more information, see https://angular-rust.github.io/guide/workspace-config#alternate-build-configurations. Setting this explicitly overrides the "--prod" flag. Aliases: -c |	string |
--cross-origin | Define the crossorigin attribute setting of elements that provide CORS support. | none\|anonymous\|use-credentials | none
--delete-output-path | Delete the output path before building. | boolean | true
--deploy-url | URL where files will be deployed. | string |
--experimental-rollup-pass | Concatenate modules with Rollup before bundling them with Webpack. | boolean | false
--extract-licenses | Extract all licenses in a separate file. |	boolean | false
--fork-type-checker | Run the type checker in a forked process. | boolean | true
--help | Shows a help message for this command in the console. | true\|false\|json | false
--i18n-missing-translation | How to handle missing translations for i18n. | warning\|error\|ignore | warning
--index | Configures the generation of the application's HTML index. | string |
--localize | Translate the bundles in one or more locales. | boolean |
--main | The full path for the main entry point to the app, relative to the current workspace. |string |
--named-chunks | Use file name for lazy loaded chunks. | boolean | true
--optimization | Enables optimization of the build output. Including minification of scripts and styles, tree-shaking, dead-code elimination, inlining of critical CSS and fonts inlining. For more information, see https://angular-rust.github.io/guide/workspace-config#optimization-configuration. | boolean | false
--output-hashing | Define the output filename cache-busting hashing mode. |	none\|all\|media\|bundles |	none
--output-path | The full path for the new output directory, relative to the current workspace. By default, writes output to a folder named dist/ in the current project. | string |
--poll | Enable and define the file watching poll time period in milliseconds. | number |
--polyfills | The full path for the polyfills file, relative to the current workspace. | string 	|
--preserve-symlinks | Do not use the real path when resolving modules. If unset then will default to true if NodeJS option --preserve-symlinks is set. | boolean |
--prod | Shorthand for "--configuration=production". Set the build configuration to the production target. By default, the production target is set up in the workspace configuration such that all builds make use of bundling, limited tree-shaking, and also limited dead code elimination. | boolean |
--progress | Log progress to the console while building. | boolean | true
--resources-output-path | The path where style resources will be placed, relative to outputPath.| string |
--service-worker | Generates a service worker config for production builds. | boolean |	false
--show-circular-dependencies | Show circular dependency warnings on builds. | boolean | true
--source-map | Output source maps for scripts and styles. For more information, see https://angular-rust.github.io/guide/workspace-config#source-map-configuration. | boolean | true
--stats-json | Generates a 'stats.json' file which can be analyzed later. | boolean | false
--subresource-integrity | Enables the use of subresource integrity validation. | boolean | false
--vendor-chunk | Generate a seperate bundle containing only vendor libraries. This option should only used for development. | boolean | true
--verbose | Adds more details to output logging. | boolean | false
--watch | Run build when files change. | boolean | false