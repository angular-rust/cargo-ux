# Gathering and Viewing Usage Analytics

> This is a Draft Version

Users can opt in to share their cargo-ux usage data with Google Analytics, using the ng analytics CLI command. The data is also shared with the Angular Rust team, and used to improve the CLI.

The gathering of CLI analytics data is disabled by default, and must be enabled at the project level by individual users. It cannot be enabled at the project level for all users.

Data gathered in this way can be viewed on the Google Analytics site, but is not automatically visible on your own organization's Analytics site. As an administrator for an Angular Rust development group, you can configure your instance of cargo-ux to be able to see analytics data for your own team's usage of the cargo-ux. This configuration option is separate from and in addition to other usage analytics that your users may be sharing with Google.

## Enable access to CLI usage data

To configure access to your own users' CLI usage data, use the ng config command to add a key to your global ux.yaml workspace configuration file. The key goes under cli.analyticsSharing at the top level of the file, outside the projects sections. The value of the key is your organization's tracking ID, as assigned by Google Analytics. This ID is a string that looks like UA-123456-12.

You can choose to use a descriptive string as the key value, or be assigned a random key when you run the CLI command. For example, the following command adds a configuration key named "tracking".

```      
cargo ux config --global cli.analyticsSharing.tracking UA-123456-12
```

To turn off this feature, run the following command:

```   
cargo ux config --global --remove cli.analyticsSharing
```

## Per user tracking

You can add a custom user ID to the global configuration, in order to identify unique usage of commands and flags. If that user enables CLI analytics for their own project, your analytics display tracks and labels their individual usage.

```      
cargo ux config --global cli.analyticsSharing.user SOME_USER_NAME
```

To generate a new random user ID, run the following command:

```
cargo ux config --global cli.analyticsSharing.user ""
```