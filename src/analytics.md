# cargo ux analytics

> This is a Draft Version

Configures the gathering of cargo-ux usage metrics. See https://angular-rust.github.io/cargo-ux/usage-analytics-gathering.

```      
cargo ux analytics <setting-or-project> <project-setting> [options]
```

## Description

The value of _settingOrProject_ is one of the following.

- "on" : Enables analytics gathering and reporting for the user.
- "off" : Disables analytics gathering and reporting for the user.
- "ci" : Enables analytics and configures reporting for use with Continuous Integration, which uses a common CI user.
- "prompt" : Prompts the user to set the status interactively.
- "project" : Sets the default status for the project to the projectSetting value, which can be any of the other values. The projectSetting argument is ignored for all other values of settingOrProject.

## Arguments
Argument             | Description | Value Type
---------------------|-------------|------------
setting-or-project | Directly enables or disables all usage analytics for the user, or prompts the user to set the status interactively, or sets the default status for the project. | on\|off\|ci\|project\|prompt
project-setting 	 | Sets the default analytics enablement status for the project.  | on\|off\|prompt

## Options

Option | Description | Value Type | Default Value
-------|-------------|------------|---------------
--help | Shows a help message for this command in the console. | true\|false\|json | false