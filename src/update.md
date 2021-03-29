# cargo ux update

> This is a Draft Version

Updates your application and its dependencies.

```      
cargo ux update [options]
```

## Description

Perform a basic update to the current stable release of the core framework and CLI by running the following command.

```   
cargo ux update @angular/cli @angular/core
```

To update to the next beta or pre-release version, use the --next option.

To update from one major version to another, use the format

```    
cargo ux update @angular/cli@^<major_version> @angular/core@^<major_version>
```

We recommend that you always update to the latest patch version, as it contains fixes we released since the initial major release. For example, use the following command to take the latest 10.x.x version and use that to update.

```
cargo ux update @angular/cli@^10 @angular/core@^10
```

For detailed information and guidance on updating your application, see the interactive Angular Rust Update Guide.

## Options
Option          | Description | Value Type | Default Value
----------------|-------------|------------|---------------
--allow-dirty 	| Whether to allow updating when the repository contains modified or untracked files. |	boolean 
--create-commits| Create source control commits for updates and migrations. **Aliases:** -C | boolean |	false
--force 	    | If false, will error out if installed packages are incompatible with the update. | boolean | false
--from 	        | Version from which to migrate from. Only available with a single package being updated, and only on migration only. | string 	
--help 	        | Shows a help message for this command in the console. | true\|false\|json | false
--migrate-only 	| Only perform a migration, do not update the installed version. | boolean 	
--next 	        | Use the prerelease version, including beta and RCs. | boolean | false
--packages 	    | The names of package(s) to update. | array 
--to 	        | Version up to which to apply migrations. Only available with a single package being updated, and only on migrations only. Requires from to be specified. Default to the installed version detected. | string 	
--verbose 	    | Display additional details about internal operations during execution. | boolean | false