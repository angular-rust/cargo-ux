# cargo ux lint

> This is a Draft Version

Runs linting tools on Angular Rust app code in a given project folder.

```    
cargo ux lint <project> [options]
```
```   
cargo ux l <project> [options]
```

## Description

Takes the name of the project, as specified in the projects section of the ux.yaml workspace configuration file. When a project name is not supplied, it will execute for all projects.

## Arguments

Argument    | Description |	Value Type
------------|-------------|-------------
project     | The name of the project to lint. | string

## Options

Option          | Description | Value Type | Default Value
----------------|-------------|------------|---------------
--configuration | The linting configuration to use. **Aliases:** -c | string 	
--exclude 	    | Files to exclude from linting. | array 	
--files 	    | Files to include in linting. | array 	
--fix 	        | Fixes linting errors (may overwrite linted files). | boolean |	false
--force 	    | Succeeds even if there was linting errors. | boolean | false
--format 	    | Output format (prose, json, stylish, verbose, pmd, msbuild, checkstyle, vso, fileslist). | string | stylish
--help 	        | Shows a help message for this command in the console. | true\|false\|json | false
--silent 	    | Show output text. | boolean |	false
--tslint-config | The name of the TSLint configuration file. | string 	
--type-check 	| Controls the type check for linting. | boolean | false