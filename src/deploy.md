# cargo ux deploy

> This is a Draft Version

Invokes the deploy builder for a specified project or for the default project in the workspace.

```    
cargo ux deploy <project> [options]
```    

## Description

The command takes an optional project name, as specified in the projects section of the ux.yaml workspace configuration file. When a project name is not supplied, executes the deploy builder for the default project.

To use the `cargo ux deploy` command, use `cargo ux add` to add a package that implements deployment capabilities to your favorite platform. Adding the package automatically updates your workspace configuration, adding a deployment CLI builder. For example:

```json   
"projects": {
  "my-project": {
    ...
    "architect": {
      ...
      "deploy": {
        "builder": "@angular/fire:deploy",
        "options": {}
      }
    }
  }
}
```

## Arguments

Argument  | Description                        | Value Type
----------|------------------------------------|------------
project   | The name of the project to deploy. | string

## Options

Option          | Description | Value Type | Default Value
----------------|-------------|------------|---------------
--configuration | One or more named builder configurations as a comma-separated list as specified in the "configurations" section of ux.yaml. The builder uses the named configurations to run the given target. For more information, see https://angular-rust.github.io/guide/workspace-config#alternate-build-configurations. **Aliases:** -c |	string
--help | Shows a help message for this command in the console. | true\|false\|json | false