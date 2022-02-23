//! **See the official documentation: <https://asdf-vm.com/manage/commands.html#all-commands>**
//!
//! # Formatted `asdf help`
//! The content on the previously referenced page is copied (with visual separation) here for your convenience.
//!
//! ```help
//! MANAGE PLUGINS
//! asdf plugin add <name> [<git-url>]      Add a plugin from the plugin repo OR,
//!                                         add a Git repo as a plugin by
//!                                         specifying the name and repo url
//!
//! asdf plugin list [--urls] [--refs]      List installed plugins. Optionally show
//!                                         git urls and git-ref
//!
//! asdf plugin list all                    List plugins registered on asdf-plugins
//!                                         repository with URLs
//!
//! asdf plugin remove <name>               Remove plugin and package versions
//!
//! asdf plugin update <name> [<git-ref>]   Update a plugin to latest commit on
//!                                         default branch or a particular git-ref
//!
//! asdf plugin update --all                Update all plugins to latest commit on
//!                                         default branch
//!
//!
//! MANAGE PACKAGES
//! asdf install                            Install all the package versions listed
//!                                         in the .tool-versions file
//!
//! asdf install <name>                     Install one tool at the version
//!                                         specified in the .tool-versions file
//!
//! asdf install <name> <version>           Install a specific version of a package
//!
//! asdf install <name> latest[:<version>]  Install the latest stable version of a
//!                                         package, or with optional version,
//!                                         install the latest stable version that
//!                                         begins with the given string
//!
//! asdf uninstall <name> <version>         Remove a specific version of a package
//!
//! asdf current                            Display current version set or being
//!                                         used for all packages
//!
//! asdf current <name>                     Display current version set or being
//!                                         used for package
//!
//! asdf where <name> [<version>]           Display install path for an installed
//!                                         or current version
//!
//! asdf which <command>                    Display the path to an executable
//!
//! asdf local <name> <version>             Set the package local version
//!
//! asdf local <name> latest[:<version>]    Set the package local version to the
//!                                         latest provided version
//!
//! asdf global <name> <version>            Set the package global version
//!
//! asdf global <name> latest[:<version>]   Set the package global version to the
//!                                         latest provided version
//!
//! asdf shell <name> <version>             Set the package version to
//!                                         `ASDF_${LANG}_VERSION` in the current shell
//!
//! asdf latest <name> [<version>]          Show latest stable version of a package
//!
//! asdf latest --all                       Show latest stable version of all the
//!                                         packages and if they are installed
//!
//! asdf list <name> [version]              List installed versions of a package and
//!                                         optionally filter the versions
//!
//! asdf list all <name> [<version>]        List all versions of a package and
//!                                         optionally filter the returned versions
//!
//! asdf help <name> [<version>]            Output documentation for plugin and tool
//!
//!
//! UTILS
//! asdf exec <command> [args...]           Executes the command shim for current version
//!
//! asdf env <command> [util]               Runs util (default: `env`) inside the
//!                                         environment used for command shim execution.
//!
//! asdf info                               Print OS, Shell and ASDF debug information.
//!
//! asdf reshim <name> <version>            Recreate shims for version of a package
//!
//! asdf shim-versions <command>            List the plugins and versions that
//!                                         provide a command
//!
//! asdf update                             Update asdf to the latest stable release
//!
//! asdf update --head                      Update asdf to the latest on the master branch
//! ```

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
