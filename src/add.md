# cargo ux add

> This is a Draft Version

Adds support for an external library to your project.

```
cargo ux add <collection> [options]
```
   
## Description

Adds the cargo package for a published library to your workspace, and configures the project in the current working directory (or the default project if you are not in a project directory) to use that library, as specified by the library's schematic. For example, adding @angular/pwa configures your project for PWA support:

```   
cargo ux add @angular/pwa
```

The default project is the value of `defaultProject` in `ux.yaml`.

## Arguments

 Argument     | Description               | Value Type 
--------------|---------------------------|------------
collection    | The package to be added.  | string

## Options
Option          | Description               | Value Type | Default Value
----------------|---------------------------|------------|---------------
--defaults      | Disable interactive input prompts for options with a default.| boolean |	
--help 	        | Shows a help message for this command in the console. | true\|false\|json | false
--interactive 	| Enable interactive input prompts. | boolean |
--registry 	    | The NPM registry to use. | string | 
--verbose 	    | Display additional details about internal operations during execution. | boolean |false