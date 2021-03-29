# cargo ux generate

> This is a Draft Version

Generates and/or modifies files based on a schematic.

```    
cargo ux generate <schematic> [options]
```
```      
cargo ux g <schematic> [options]
```
## Arguments
Argument 	    | Description |	Value Type
----------------|-------------|------------
schematic   	| The schematic or collection:schematic to generate.
This option can take one of the following sub-commands: `app-shell`, `application`, `class`, `component`, `directive`, `enum`, `guard`, `interceptor`, `interface`, `library` , `module`, `pipe` , `resolver`, `service`, `service-worker`, `web-worker` | string

## Options

Option 	        | Description | Value Type | Default Value
----------------|-------------|------------|----------------
--defaults 	    | Disable interactive input prompts for options with a default. | boolean 	
--dry-run 	    | Run through and reports activity without writing out results. **Aliases:** -d | boolean |	false
--force 	    | Force overwriting of existing files. **Aliases:** -f | boolean | false
--help 	        | Shows a help message for this command in the console. | true\|false\|json |	false
--interactive 	| Enable interactive input prompts. | boolean 	

## Schematic commands

### app-shell

```   
cargo ux generate app-shell [options]
```
```   
cargo ux g app-shell [options]
```
Generates an app shell for running a server-side version of an app.

#### Options
Option                      | Description | Value Type | Default Value
----------------------------|-------------|------------|--------------
--app-dir 	                | The name of the application directory. | string |	app
--app-id 	                | The app ID to use in withServerTransition(). | string | serverApp
--client-project 	        | The name of the related client app. | string 	
--main 	                    | The name of the main entry-point file. | string | main.server.rs
--root-module-class-name 	| The name of the root module class. | string | AppServerModule
--root-module-file-name 	| The name of the root module file | string | app.server.module.rs
--route 	                | Route path used to produce the app shell. | string | shell

### application

```   
cargo ux generate application <name> [options]
```
```   
cargo ux g application <name> [options]
```

Generates a new basic app definition in the "projects" subfolder of the workspace.

#### Arguments
Argument | Description |	Value Type
---------|-------------|-------------
name     | The name of the new app. | string

#### Options

Option 	            | Description | Value Type | Default Value
--------------------|-------------|------------|--------------
--inline-style 	    | Include styles inline in the root component.rs file. Only CSS styles can be included inline. Default is false, meaning that an external styles file is created and referenced in the root component.rs file. **Aliases:** -s | boolean 	
--inline-template 	| Include template inline in the root component.rs file. Default is false, meaning that an external template file is created and referenced in the root component.rs file. **Aliases:** -t | boolean 	
--legacy-browsers   | Add support for legacy browsers like Internet Explorer using differential loading. | boolean |false
--minimal 	        | Create a bare-bones project without any testing frameworks. (Use for learning purposes only.) | boolean |	false
--prefix 	        | A prefix to apply to generated selectors. **Aliases:** -p | string | app
--routing 	        | Create a routing module. | boolean | false
--skip-install 	    | Skip installing dependency packages. | boolean | false
--skip-package-json | Do not add dependencies to the "package.yaml" file. | boolean | false
--skip-tests 	    | Do not create "spec.rs" test files for the application. **Aliases:** -S | boolean | false
--strict            | Creates an application with stricter bundle budgets settings. | boolean | false
--view-encapsulation| The view encapsulation strategy to use in the new app. | Emulated\|None\|\ShadowDom 	

### class

```     
cargo ux generate class <name> [options]
```
```   
cargo ux g class <name> [options]
```

Creates a new generic class definition in the given or default project.

#### Arguments

Argument | Description | Value Type
---------|-------------|-----------
name     | The name of the new class. | string

#### Options

Option 	        | Description |	Value Type | Default Value
----------------|-------------|------------|---------------
--project 	    | The name of the project. | string 	
--skip-tests 	| Do not create "spec.rs" test files for the new class. | boolean |	false
--type 	        | Adds a developer-defined type to the filename, in the format "name.type.rs". | string

### component

```  
cargo ux generate component <name> [options]
```
```   
cargo ux g component <name> [options]
```

Creates a new generic component definition in the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|-----------
name     | The name of the component. | string

#### Options

Option 	            | Description | Value Type | Default Value
--------------------|-------------|------------|--------------
--change-detection 	| The change detection strategy to use in the new component. **Aliases:** -c | Default\|OnPush | Default
--display-block 	| Specifies if the style will contain :host { display: block; }. **Aliases:** -b | boolean | false
--export 	        | The declaring module exports this component. | boolean | false
--flat              | Create the new files at the top level of the current project. | boolean | false
--inline-style 	    | Include styles inline in the component.rs file. Only CSS styles can be included inline. By default, an external styles file is created and referenced in the component.rs file. **Aliases:** -s | boolean | false
--inline-template 	| Include template inline in the component.rs file. By default, an external template file is created and referenced in the component.rs file. **Aliases:** -t | boolean |	false
--module 	        | The declaring module. **Aliases:** -m | string 	
--prefix 	        | The prefix to apply to the generated component selector. **Aliases:** -p | string 	
--project 	        | The name of the project. | string 	
--selector 	        | The HTML selector to use for this component. | string 	
--skip-import 	    | Do not import this component into the owning module. | boolean | false
--skip-selector 	| Specifies if the component should have a selector or not. | boolean |	false
--skip-tests 	    | Do not create "spec.rs" test files for the new component. | boolean |	false
--type 	            | Adds a developer-defined type to the filename, in the format "name.type.rs". | string |Component
--view-encapsulation | The view encapsulation strategy to use in the new component. **Aliases:** -v | Emulated\|None\|ShadowDom 	

### directive

```
cargo ux generate directive <name> [options]
```
```   
cargo ux g directive <name> [options]
```

Creates a new generic directive definition in the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|-------------
name     | The name of the new directive. | string

#### Options

Option 	        | Description | Value Type | Default Value
----------------|-------------|------------|---------------
--export 	    | The declaring module exports this directive. | boolean | false
--flat 	        | When true (the default), creates the new files at the top level of the current project. | boolean | true
--module 	    | The declaring module. **Aliases:** -m | string 	
--prefix 	    | A prefix to apply to generated selectors. **Aliases:** -p | string 	
--project 	    | The name of the project. | string 	
--selector 	    | The HTML selector to use for this directive. | string
--skip-import 	| Do not import this directive into the owning module. | boolean | false
--skip-tests 	| Do not create "spec.rs" test files for the new class. | boolean | false

### enum

```
cargo ux generate enum <name> [options]
```
```   
cargo ux g enum <name> [options]
```

Generates a new, generic enum definition for the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|------------
name     | The name of the enum. | string

#### Options

Option      | Description |	Value Type | Default Value
--project 	| The name of the project in which to create the enum. Default is the configured default project for the workspace. | string 	

### guard

```  
cargo ux generate guard <name> [options]
```
```   
cargo ux g guard <name> [options]
```

Generates a new, generic route guard definition in the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|------------
name     | The name of the new route guard. | string

#### Options

Option 	        | Description |	Value Type | Default Value
----------------|-------------|------------|---------------
--flat 	        | When true (the default), creates the new files at the top level of the current project. | boolean | true
--implements    | Specifies which interfaces to implement. | array 	
--project       | The name of the project. | string 	
--skip-tests 	| Do not create "spec.rs" test files for the new guard. | boolean |	false

### interceptor

```  
cargo ux generate interceptor <name> [options]
```
```
cargo ux g interceptor <name> [options]
```

Creates a new, generic interceptor definition in the given or default project.

#### Arguments
Argument | Description | Value Type
name     | The name of the interceptor. | string

#### Options

Option 	        | Description | Value Type | Default Value
----------------|-------------|------------|----------------
--flat 	        | When true (the default), creates files at the top level of the project. | boolean | true
--project 	    | The name of the project. | string 	
--skip-tests 	| Do not create "spec.rs" test files for the new interceptor. | boolean | false

### interface

```  
cargo ux generate interface <name> <type> [options]
```
```
cargo ux g interface <name> <type> [options]
```

Creates a new generic interface definition in the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|------------
name     | The name of the interface. | string
type     | Adds a developer-defined type to the filename, in the format "name.type.rs". | string

#### Options

Option 	    | Description | Value Type | Default Value
------------|-------------|------------|--------------
--prefix 	| A prefix to apply to generated selectors. | string 	
--project 	| The name of the project. | string 	

### library

```  
cargo ux generate library <name> [options]
```
```
cargo ux g library <name> [options]
```

Creates a new generic library project in the current workspace.

#### Arguments
Argument | Description | Value Type
---------|-------------|-------------
name     | The name of the library. | string

#### Options

Option 	            | Description | Value Type | Default Value
--------------------|-------------|------------|--------------
--entry-file 	    | The path at which to create the library's public API file, relative to the workspace root. | string |	public-api
--prefix 	        | A prefix to apply to generated selectors. **Aliases:** -p | string | lib
--skip-install 	    | Do not install dependency packages. | boolean | false
--skip-package-yaml | Do not add dependencies to the "package.yaml" file. | boolean | false

### module

```  
cargo ux generate module <name> [options]
```
```   
cargo ux g module <name> [options]
```

Creates a new generic module definition in the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|------------
name     | The name of the module. | string

#### Options

Option 	    | Description |	Value Type | Default Value
------------|-------------|------------|---------------
--flat 	    | Create the new files at the top level of the current project root. | boolean | false
--module 	| The declaring module. **Aliases:** -m | string 	
--project 	| The name of the project. | string 	
--route 	| The route path for a lazy-loaded module. When supplied, creates a component in the new module, and adds the route to that component in the Routes array declared in the module provided in the --module option. | string 	
--routing 	| Create a routing module. | boolean | false
--routing-scope | The scope for the new routing module. | Child\|Root |	Child

### pipe

```  
cargo ux generate pipe <name> [options]
```
```    
cargo ux g pipe <name> [options]
```

Creates a new generic pipe definition in the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|------------
name     | The name of the pipe. | string

#### Options

Option 	        | Description | Value Type | Default Value
----------------|-------------|------------|----------------
--export 	    | The declaring module exports this pipe. | boolean |	false
--flat 	        | When true (the default) creates files at the top level of the project. | boolean | true
--module 	    | The declaring module. **Aliases:** -m | string 	
--project 	    | The name of the project. | string 	
--skip-import   | Do not import this pipe into the owning module. | boolean |	false
--skip-tests 	| Do not create "spec.rs" test files for the new pipe. | boolean | false

### resolver

```
cargo ux generate resolver <name> [options]
```
```   
cargo ux g resolver <name> [options]
```

Generates a new, generic resolver definition in the given or default project.

#### Arguments

Argument | Description | Value Type
---------|-------------|------------
name     | The name of the new resolver. | string

#### Options
Option      | Description |	Value Type | Default Value
--flat 	    | When true (the default), creates the new files at the top level of the current project. | boolean | true
--project 	| The name of the project. | string 	
--skip-tests| Do not create "spec.rs" test files for the new resolver. | boolean | false

### service

```  
cargo ux generate service <name> [options]
```
```   
cargo ux g service <name> [options]
```

Creates a new, generic service definition in the given or default project.

#### Arguments
Argument | Description | Value Type
---------|-------------|------------
name     | The name of the service. | string

#### Options

Option 	    | Description | Value Type | Default Value
------------|-------------|------------|----------------
--flat 	    | When true (the default), creates files at the top level of the project. | boolean | true
--project   | The name of the project. | string 	
--skip-tests| Do not create "spec.rs" test files for the new service. | boolean | false

### service-worker

```  
cargo ux generate service-worker [options]
```
```   
cargo ux g service-worker [options]
```

Pass this schematic to the "run" command to create a service worker

#### Options

Option 	        | Description | Value Type | Default Value
----------------|-------------|------------|---------------
--configuration | The configuration to apply service worker to. | string | production
--project 	    | The name of the project. | string
--target 	    | The target to apply service worker to. | string |	build

### web-worker

```  
cargo ux generate web-worker <name> [options]
```
``` 
cargo ux g web-worker <name> [options]
``` 

Creates a new generic web worker definition in the given or default project.

#### Arguments

Argument | Description | Value Type
name     | The name of the worker. | string

#### Options

Option 	    | Description |	Value Type | Default Value
------------|-------------|------------|---------------
--project 	| The name of the project. | string 	
--snippet 	| Add a worker creation snippet in a sibling file of the same name. | boolean |	true
--target 	| The target to apply web worker to. | string | build