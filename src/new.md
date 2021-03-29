# cargo ux new

> This is a Draft Version

Creates a new workspace and an initial Angular Rust application.

```    
cargo ux new <name> [options]
```
```   
cargo ux n <name> [options]
```

## Description

Creates and initializes a new Angular Rust application that is the default project for a new workspace.

Provides interactive prompts for optional configuration, such as adding routing support. All prompts can safely be allowed to default.

- The new workspace folder is given the specified project name, and contains configuration files at the top level.

- By default, the files for a new initial application (with the same name as the workspace) are placed in the src/ subfolder. Corresponding end-to-end tests are placed in the e2e/ subfolder.

- The new application's configuration appears in the projects section of the ux.yaml workspace configuration file, under its project name.

- Subsequent applications that you generate in the workspace reside in the projects/ subfolder.

If you plan to have multiple applications in the workspace, you can create an empty workspace by setting the --createApplication option to false. You can then use `cargo ux generate application` to create an initial application. This allows a workspace name different from the initial app name, and ensures that all applications reside in the /projects subfolder, matching the structure of the configuration file.

## Arguments

Argument |	Description 	Value Type
name     | The name of the new workspace and initial project. | string

## Options

Option                  | Description | Value Type | Default Value
------------------------|-------------|------------|----------------
--collection 	        | A collection of schematics to use in generating the initial application. **Aliases:** -c | string 	
--commit 	            | Initial git repository commit information. | boolean | true
--create-application 	| Create a new initial application project in the 'src' folder of the new workspace. When false, creates an empty workspace with no initial application. You can then use the generate application command so that all applications are created in the projects folder. | boolean |	true
--defaults 	            | Disable interactive input prompts for options with a default. |	boolean 
--directory 	        | The directory name to create the workspace in. | string 	
--dry-run 	            | Run through and reports activity without writing out results. **Aliases:** -d | boolean |	false
--force 	            | Force overwriting of existing files. **Aliases:** -f | boolean | false
--help 	                | Shows a help message for this command in the console. | true\|false\|json | false
--inline-style 	        | Include styles inline in the component file. By default, an external styles file is created and referenced in the component file. **Aliases:** -s | boolean 	
--inline-template 	    | Include template inline in the component file. By default, an external template file is created and referenced in the component file. **Aliases:** -t | boolean 	
--interactive           | Enable interactive input prompts. | boolean 	
--legacy-browsers 	    | Add support for legacy browsers like Internet Explorer using differential loading. | boolean | false
--minimal 	            | Create a workspace without any testing frameworks. (Use for learning purposes only.) | boolean | false
--new-project-root 	    | The path where new projects will be created, relative to the new workspace root. | string | projects
--prefix 	            | The prefix to apply to generated selectors for the initial project. **Aliases:** -p | string | app
--routing 	            | Generate a routing module for the initial project. | boolean 	
--skip-git 	            | Do not initialize a git repository. **Aliases:** -g | boolean | false
--skip-install 	        | Do not install dependency packages. | boolean | false
--skip-tests 	        | Do not generate "spec.rs" test files for the new project. **Aliases:** -S | boolean |	false
--strict 	            | Creates a workspace with stricter type checking and stricter bundle budgets settings. This setting helps improve maintainability and catch bugs ahead of time. For more information, see https://angular-rust.github.io/guide/strict-mode | boolean |	false
--verbose 	            | Add more details to output logging. **Aliases:** -v | boolean | false
--view-encapsulation 	| The view encapsulation strategy to use in the initial project. | Emulated\|None\|ShadowDom