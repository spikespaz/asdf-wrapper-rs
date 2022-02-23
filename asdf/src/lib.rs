//! **See the official documentation: <https://asdf-vm.com/manage/commands.html#all-commands>**
//!
//! # Commands and Exports
//!
//! The content on the previously referenced page is copied (with visual separation) here for your convenience.
//!
//! ## MANAGE PLUGINS
//!
//! | Command and Exports | Description |
//! | ------------------- | ----------- |
//! | `asdf plugin add <name> [<git-url>]`    <br/><sub>[`asdf::plugins::add`],<br/>`asdf::prelude::asdf_plugin_add`</sub>               | Add a plugin from the plugin repo OR, add a Git repo as a plugin by specifying the name and repo url |
//! | `asdf plugin list [--urls] [--refs]`    <br/><sub>[`asdf::plugins::list`],<br/>`asdf::prelude::asdf_plugin_list`</sub>             | List installed plugins. Optionally show git urls and git-ref                                         |
//! | `asdf plugin list all`                  <br/><sub>[`asdf::plugins::list_all`],<br/>`asdf::prelude::asdf_plugin_list_all`</sub>     | List plugins registered on asdf-plugins repository with URLs                                         |
//! | `asdf plugin remove <name>`             <br/><sub>[`asdf::plugins::remove`],<br/>`asdf::prelude::asdf_plugin_remove`</sub>         | Remove plugin and package versions                                                                   |
//! | `asdf plugin update <name> [<git-ref>]` <br/><sub>[`asdf::plugins::update`],<br/>`asdf::prelude::asdf_plugin_update`</sub>         | Update a plugin to latest commit on default branch or a particular git-ref                           |
//! | `asdf plugin update --all`              <br/><sub>[`asdf::plugins::update_all`],<br/>`asdf::prelude::asdf_plugin_update_all`</sub> | Update all plugins to latest commit on default branch                                                |
//!
//! ## MANAGE PACKAGES
//!
//! | Command and Exports | Description |
//! | ------------------- | ----------- |
//! | `asdf install`                           <br/><sub>[`asdf::packages::install`],<br/>`asdf::prelude::asdf_install`</sub>       | Install all the package versions listed in the .tool-versions file                                                                            |
//! | `asdf install <name>`                    <br/><sub>[`asdf::packages::install`],<br/>`asdf::prelude::asdf_install`</sub>       | Install one tool at the version specified in the .tool-versions file                                                                          |
//! | `asdf install <name> <version>`          <br/><sub>[`asdf::packages::install`],<br/>`asdf::prelude::asdf_install`</sub>       | Install a specific version of a package                                                                                                       |
//! | `asdf install <name> latest[:<version>]` <br/><sub>[`asdf::packages::install`],<br/>`asdf::prelude::asdf_install`</sub>       | Install the latest stable version of a package, or with optional version, install the latest stable version that begins with the given string |
//! | `asdf uninstall <name> <version>`        <br/><sub>[`asdf::packages::uninstall`],<br/>`asdf::prelude::asdf_uninstall`</sub>   | Remove a specific version of a package                                                                                                        |
//! | `asdf current`                           <br/><sub>[`asdf::packages::current`],<br/>`asdf::prelude::asdf_current`</sub>       | Display current version set or being used for all packages                                                                                    |
//! | `asdf current <name>`                    <br/><sub>[`asdf::packages::current`],<br/>`asdf::prelude::asdf_current`</sub>       | Display current version set or being used for package                                                                                         |
//! | `asdf where <name> [<version>]`          <br/><sub>[`asdf::packages::locate`],<br/>`asdf::prelude::asdf_where`</sub>          | Display install path for an installed or current version                                                                                      |
//! | `asdf which <command>`                   <br/><sub>[`asdf::packages::which`],<br/>`asdf::prelude::asdf_which`</sub>           | Display the path to an executable                                                                                                             |
//! | `asdf local <name> <version>`            <br/><sub>[`asdf::packages::local`],<br/>`asdf::prelude::asdf_local`</sub>           | Set the package local version                                                                                                                 |
//! | `asdf local <name> latest[:<version>]`   <br/><sub>[`asdf::packages::local`],<br/>`asdf::prelude::asdf_local`</sub>           | Set the package local version to the latest provided version                                                                                  |
//! | `asdf global <name> <version>`           <br/><sub>[`asdf::packages::global`],<br/>`asdf::prelude::asdf_global`</sub>         | Set the package global version                                                                                                                |
//! | `asdf global <name> latest[:<version>]`  <br/><sub>[`asdf::packages::global`],<br/>`asdf::prelude::asdf_global`</sub>         | Set the package global version to the latest provided version                                                                                 |
//! | `asdf shell <name> <version>`            <br/><sub>[`asdf::packages::shell`],<br/>`asdf::prelude::asdf_shell`</sub>           | Set the package version to `ASDF_${LANG}_VERSION` in the current shell                                                                        |
//! | `asdf latest <name> [<version>]`         <br/><sub>[`asdf::packages::latest`],<br/>`asdf::prelude::asdf_latest`</sub>         | Show latest stable version of a package                                                                                                       |
//! | `asdf latest --all`                      <br/><sub>[`asdf::packages::latest_all`],<br/>`asdf::prelude::asdf_latest_all`</sub> | Show latest stable version of all the packages and if they are installed                                                                      |
//! | `asdf list <name> [version]`             <br/><sub>[`asdf::packages::list`],<br/>`asdf::prelude::asdf_list`</sub>             | List installed versions of a package and optionally filter the versions                                                                       |
//! | `asdf list all <name> [<version>]`       <br/><sub>[`asdf::packages::list_all`],<br/>`asdf::prelude::asdf_list_all`</sub>     | List all versions of a package and optionally filter the returned versions                                                                    |
//! | `asdf help <name> [<version>]`           <br/><sub>[`asdf::utils::help`],<br/>`asdf::prelude::asdf_help`</sub>                | Output documentation for plugin and tool                                                                                                      |
//!
//!
//! ## UTILS
//!
//! | Command and Exports | Description |
//! | ------------------- | ----------- |
//! | `asdf exec <command> [args...]` <br/><sub>[`asdf::utils::exec`],<br/>`asdf::prelude::asdf_exec`</sub>                   | Executes the command shim for current version                                     |
//! | `asdf env <command> [util]`     <br/><sub>[`asdf::utils::env`],<br/>`asdf::prelude::asdf_env`</sub>                     | Runs util (default: `env`) inside the environment used for command shim execution |
//! | `asdf info`                     <br/><sub>[`asdf::utils::info`],<br/>`asdf::prelude::asdf_info`</sub>                   | Print OS, Shell and ASDF debug information                                        |
//! | `asdf reshim <name> <version>`  <br/><sub>[`asdf::utils::reshim`],<br/>`asdf::prelude::asdf_reshim`</sub>               | Recreate shims for version of a package                                           |
//! | `asdf shim-versions <command>`  <br/><sub>[`asdf::utils::shim_versions`],<br/>`asdf::prelude::asdf_shim_versions`</sub> | List the plugins and versions that provide a command                              |
//! | `asdf update`                   <br/><sub>[`asdf::utils::update`],<br/>`asdf::prelude::asdf_update`</sub>               | Update asdf to the latest stable release                                          |
//! | `asdf update --head`            <br/><sub>[`asdf::utils::update`],<br/>`asdf::prelude::asdf_update`</sub>               | Update asdf to the latest on the master branch                                    |

pub mod asdf;

pub use self::asdf::{packages, plugins, utils};

/// The 'prelude' module can be imported as `use asdf::prelude::*` if you want access to a flat API.
/// The functions are renamed according to the corresponding command, formatted in `snake_case`.
pub mod prelude {
    use super::*;

    pub use plugins::add as asdf_plugin_add;
    pub use plugins::list as asdf_plugin_list;
    pub use plugins::list_all as asdf_plugin_list_all;
    pub use plugins::remove as asdf_plugin_remove;
    pub use plugins::update as asdf_plugin_update;
    pub use plugins::update_all as asdf_plugin_update_all;

    pub use packages::current as asdf_current;
    pub use packages::global as asdf_global;
    pub use packages::install as asdf_install;
    pub use packages::latest as asdf_latest;
    pub use packages::latest_all as asdf_latest_all;
    pub use packages::list as asdf_list;
    pub use packages::list_all as asdf_list_all;
    pub use packages::local as asdf_local;
    pub use packages::locate as asdf_where;
    pub use packages::shell as asdf_shell;
    pub use packages::uninstall as asdf_uninstall;
    pub use packages::which as asdf_which;

    pub use utils::env as asdf_env;
    pub use utils::exec as asdf_exec;
    pub use utils::help as asdf_help;
    pub use utils::info as asdf_info;
    pub use utils::reshim as asdf_reshim;
    pub use utils::shim_versions as asdf_shim_versions;
    pub use utils::update as asdf_update;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
