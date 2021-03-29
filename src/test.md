# cargo ux test

> This is a Draft Version

Runs unit tests in a project.

```      
cargo ux test <project> [options]
```
```   
cargo ux t <project> [options]
```

## Description

Takes the name of the project, as specified in the projects section of the ux.yaml workspace configuration file. When a project name is not supplied, it will execute for all projects.

## Arguments
Argument 	| Description | Value Type
------------|-------------|-----------
project     | The name of the project to build. Can be an application or a library. | string

## Options

Option                  | Description | Value Type | Default Value
------------------------|-------------|------------|---------------
--browsers 	            | Override which browsers tests are run against. | string 	
--code-coverage         | Output a code coverage report. | boolean | false
--code-coverage-exclude | Globs to exclude from code coverage. | array 	
--configuration 	    | One or more named builder configurations as a comma-separated list as specified in the "configurations" section of ux.yaml. The builder uses the named configurations to run the given target. For more information, see https://angular-rust.github.io/guide/workspace-config#alternate-build-configurations. Setting this explicitly overrides the "--prod" flag. **Aliases:** -c | string 	
--help 	                | Shows a help message for this command in the console.| true\|false\|json | false
--include 	            | Globs of files to include, relative to workspace or project root. There are 2 special cases: - when a path to directory is provided, all spec files ending ".spec. @(ts\|tsx)" will be included - when a path to a file is provided, and a matching spec file exists it will be included instead | array
--karma-config 	        | The name of the Karma configuration file. | string 	
--main 	                | The name of the main entry-point file. | string 	
--poll 	                | Enable and define the file watching poll time period in milliseconds. | number
--polyfills 	        | The name of the polyfills file. | string 	
--preserve-symlinks 	| Do not use the real path when resolving modules. If unset then will default to true if NodeJS option --preserve-symlinks is set. | boolean 	
--prod 	                | Shorthand for "--configuration=production". Set the build configuration to the production target. By default, the production target is set up in the workspace configuration such that all builds make use of bundling, limited tree-shaking, and also limited dead code elimination. | boolean 	
--progress 	            | Log progress to the console while building. | boolean | true
--reporters 	        | Karma reporters to use. Directly passed to the karma runner. | array 	
--source-map 	        | Output source maps for scripts and styles. For more information, see https://angular-rust.github.io/guide/workspace-config#source-map-configuration. | boolean | true
--watch 	            | Run build when files change. | boolean 