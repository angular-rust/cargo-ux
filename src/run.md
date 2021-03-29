# cargo ux run

> This is a Draft Version

Runs an Architect target with an optional custom builder configuration defined in your project.

```    
cargo ux run <target> [options]
```

## Description

Architect is the tool that the CLI uses to perform complex tasks such as compilation, according to provided configurations. The CLI commands run Architect targets such as build, serve, test, and lint. Each named target has a default configuration, specified by an "options" object, and an optional set of named alternate configurations in the "configurations" object.

For example, the "serve" target for a newly generated app has a predefined alternate configuration named "production".

You can define new targets and their configuration options in the "architect" section of the ux.yaml file. If you do so, you can run them from the command line using the `ng run` command. Execute the command using the following format.

```   
cargo ux run project:target[:configuration]
```

## Arguments

Argument  | Description | Value Type
----------|-------------|------------
target    | The Architect target to run. | string

## Options

Option          | Description | Value Type | Default Value
----------------|-------------|------------|----------------
--configuration | One or more named builder configurations as a comma-separated list as specified in the "configurations" section of ux.yaml. The builder uses the named configurations to run the given target. For more information, see https://angular-rust.github.io/guide/workspace-config#alternate-build-configurations. **Aliases:** -c | string 	
--help          | Shows a help message for this command in the console. | true\|false\|json | false