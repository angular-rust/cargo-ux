# cargo ux e2e

> This is a Draft Version

Builds and serves an Angular Rust app, then runs end-to-end tests using Protractor.
```    
cargo ux e2e <project> [options]
```
```   
cargo ux e <project> [options]
```

## Description

Must be executed from within a workspace directory. When a project name is not supplied, it will execute for all projects.

## Arguments
Argument    | Description | Value Type
------------|-------------|-------------
project     | The name of the project to build. Can be an application or a library. | string

## Options

Option              | Description | Value Type | Default Value
--------------------|-------------|------------|---------------
--base-url          | Base URL for protractor to connect to. | string 	
--configuration 	| One or more named builder configurations as a comma-separated list as specified in the "configurations" section of ux.yaml. The builder uses the named configurations to run the given target. For more information, see https://angular-rust.github.io/guide/workspace-config#alternate-build-configurations. Setting this explicitly overrides the "--prod" flag. **Aliases:** -c | string |
--dev-server-target | A dev-server builder target to run tests against in the format of project:target[:configuration]. You can also pass in more than one configuration name as a comma-separated list. Example: project:target:production,staging. | string |
--grep              | Execute specs whose names match the pattern, which is internally compiled to a RegExp. | string 	
--help              | Shows a help message for this command in the console. |  true\|false\|json | false
--host 	            | Host to listen on. | string |
--invert-grep       | Invert the selection specified by the 'grep' option. | | boolean | false
--port              | The port to use to serve the application. | number | 	
--prod 	            | Shorthand for "--configuration=production". Set the build configuration to the production target. By default, the production target is set up in the workspace configuration such that all builds make use of bundling, limited tree-shaking, and also limited dead code elimination. | boolean |
--protractor-config | The name of the Protractor configuration file. | string |
--specs 	        | Override specs in the protractor config. | array |
--suite 	        | Override suite in the protractor config. | string |
--webdriver-update 	| Try to update webdriver. | boolean | true