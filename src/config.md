# cargo ux config

> This is a Draft Version

Retrieves or sets Angular Rust configuration values in the ux.yaml file for the workspace.

```      
cargo ux config <json-path> <value> [options]
```

## Arguments

Argument      | Description | Value Type
--------------|-------------|------------
json-path     | The configuration key to set or query, in JSON path format. For example: "a[3].foo.bar[2]". If no new value is provided, returns the current value of this key. | string
value         | If provided, a new value for the given configuration key. | string

## Options

Option    | Description | Value Type | Default Value
----------|-------------|------------|---------------
--global  | Access the global configuration in the caller's home directory. **Aliases:** -g | boolean | false
--help    | Shows a help message for this command in the console. | true\|false\|json | false