# cargo ux serve

> This is a Draft Version

Builds and serves your app, rebuilding on file changes.

```    
cargo ux serve <project> [options]
```
```      
cargo ux s <project> [options]
```

## Arguments
Argument    | Description | Value Type
------------|-------------|-----------
project  	| The name of the project to build. Can be an application or a library. | string

## Options
Option 	            | Description | Value Type | Default Value
--------------------|-------------|------------|---------------
--allowed-hosts 	| List of hosts that are allowed to access the dev server. | array 	
--browser-target 	| A browser builder target to serve in the format of project:target[:configuration]. You can also pass in more than one configuration name as a comma-separated list. Example: project:target:production,staging. | string 	
--configuration 	| One or more named builder configurations as a comma-separated list as specified in the "configurations" section of ux.yaml. The builder uses the named configurations to run the given target. For more information, see https://angular-rust.github.io/guide/workspace-config#alternate-build-configurations. Setting this explicitly overrides the "--prod" flag. **Aliases:** -c | string 	
--disable-host-check| Don't verify connected clients are part of allowed hosts. | boolean | false
--help              | Shows a help message for this command in the console. | true\|false\|json | false
--hmr 	            | Enable hot module replacement. | boolean | false
--host 	            | Host to listen on. | string |	localhost
--live-reload 	    | Whether to reload the page on change, using live-reload. | boolean | true
--open 	            | Opens the url in default browser. **Aliases:** -o | boolean |	false
--poll 	            | Enable and define the file watching poll time period in milliseconds. | 	number 	
--port 	            | Port to listen on. | number |	4200
--prod 	            | Shorthand for "--configuration=production". Set the build configuration to the production target. By default, the production target is set up in the workspace configuration such that all builds make use of bundling, limited tree-shaking, and also limited dead code elimination. | boolean 	
--proxy-config 	    | Proxy configuration file. | string 	
--public-host 	    | The URL that the browser client (or live-reload client, if enabled) should use to connect to the development server. Use for a complex dev server setup, such as one with reverse proxies. | string 	
--serve-path 	    | The pathname where the app will be served. | string 	
--ssl 	            | Serve using HTTPS. | boolean | false
--ssl-cert 	        | SSL certificate to use for serving HTTPS. | string 	
--ssl-key 	        | SSL key to use for serving HTTPS. | string 	
only used for development. | boolean 	
--verbose 	        | Adds more details to output logging. | boolean 	
--watch 	        | Rebuild on change. | boolean | true