# cargo ux doc

> This is a Draft Version

Opens the official Angular Rust documentation (angular-rust.github.io) in a browser, and searches for a given keyword.

```
cargo ux doc <keyword> [options]
```
```   
cargo ux d <keyword> [options]
```

## Arguments
Argument  | Description | Value Type
----------|-------------|------------
keyword   | The keyword to search for, as provided in the search bar in angular-rust.github.io. | string

## Options
Option   | Description | Value Type | Default Value
---------|-------------|------------|--------------
--help   | Shows a help message for this command in the console. | true\|false\|json | false
--search | Search all of angular-rust.github.io. Otherwise, searches only API reference documentation. **Aliases:** -s | boolean | false
--version| Contains the version of Angular Rust to use for the documentation. If not provided, the command uses your current Angular Rust core version. | number |