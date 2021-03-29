# cargo ux extract-i18n

> This is a Draft Version

Extracts i18n messages from source code.

```    
cargo ux extract-i18n <project> [options]
```
```   
cargo ux i18n-extract <project> [options]
```
```   
cargo ux xi18n <project> [options]
```
 
## Arguments
Argument    | Description |	Value Type
------------|-------------|------------
project     | The name of the project to build. Can be an application or a library. | string

## Options

Option           | Description | Value Type | Default Value
-----------------|-------------|------------|---------------
--browser-target | A browser builder target to extract i18n messages in the format of project:target[:configuration]. You can also pass in more than one configuration name as a comma-separated list. Example: project:target:production,staging. | string |
--configuration  | One or more named builder configurations as a comma-separated list as specified in the "configurations" section of ux.yaml. The builder uses the named configurations to run the given target. For more information, see https://angular-rust.github.io/guide/workspace-config#alternate-build-configurations. Setting this explicitly overrides the "--prod" flag. **Aliases:** -c | string 	
--format 	    | Output format for the generated file. | xmb\|xlf\|xlif\|xliff\|xlf2\|xliff2\|json\|arb | xlf
--help 	        | Shows a help message for this command in the console. | true\|false\|json | false
--ivy 	        | Use Ivy compiler to extract translations. The default for Ivy applications. | boolean 	
--out-file 	    | Name of the file to output. | string 	
--output-path 	| Path where output will be placed. | string 	
--prod 	        | Shorthand for "--configuration=production". Set the build configuration to the production target. By default, the production target is set up in the workspace configuration such that all builds make use of bundling, limited tree-shaking, and also limited dead code elimination. | boolean 	
--progress 	    | Log progress to the console. | boolean | true