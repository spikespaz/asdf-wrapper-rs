//! All `asdf` commands have been separated into modules based on how the help output categorizes them.
//!
//! ## Note:
//!
//! There are two important differences between how they are categorized by the help output and this crate.
//!
//! - The `asdf help` command is provided by `asdf::utils::help` rather than `asdf::packages::help` as the documentation would suggest.
//! - Due to `where` being a Rust keyword, the `asdf which` command is provided by `asdf::packages::locate`.
//!   - This is unless you use the flat API, in which case it is re-exported to `asdf::prelude::asdf_where`.

pub enum Response {
    AsdfNotAvailable,
}

pub type Result<T> = std::result::Result<T, Response>;

/// MANAGE PLUGINS
pub mod plugins {
    use super::{Response, Result};

    /// ```help
    /// asdf plugin add <name> [<git-url>]      Add a plugin from the plugin repo OR,
    ///                                         add a Git repo as a plugin by
    ///                                         specifying the name and repo url
    /// ```
    pub fn add<A, B>(name: A, git_url: Option<B>) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf plugin list [--urls] [--refs]      List installed plugins. Optionally show
    ///                                         git urls and git-ref
    /// ```
    pub fn list(git_urls: bool, git_refs: bool) -> ! {
        todo!();
    }

    /// ```help
    /// asdf plugin list all                    List plugins registered on asdf-plugins
    ///                                         repository with URLs
    /// ```
    pub fn list_all() -> ! {
        todo!();
    }

    /// ```help
    /// asdf plugin remove <name>               Remove plugin and package versions
    /// ```
    pub fn remove<A>(name: A) -> !
    where
        A: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf plugin update <name> [<git-ref>]   Update a plugin to latest commit on
    ///                                         default branch or a particular git-ref
    /// ```
    pub fn update<A, B>(name: A, git_ref: B) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf plugin update --all                Update all plugins to latest commit on
    ///                                         default branch
    /// ```
    pub fn update_all() -> ! {
        todo!();
    }
}

/// MANAGE PACKAGES
pub mod packages {
    use super::{Response, Result};

    /// ```help
    /// asdf install                            Install all the package versions listed
    ///                                         in the .tool-versions file
    /// asdf install <name>                     Install one tool at the version
    ///                                         specified in the .tool-versions file
    /// asdf install <name> <version>           Install a specific version of a package
    /// asdf install <name> latest[:<version>]  Install the latest stable version of a
    ///                                         package, or with optional version,
    ///                                         install the latest stable version that
    ///                                         begins with the given string
    /// ```
    pub fn install<A, B>(name: Option<A>, version: Option<B>) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf uninstall <name> <version>         Remove a specific version of a package
    /// ```
    pub fn uninstall<A, B>(name: A, version: B) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf current                            Display current version set or being
    ///                                         used for all packages
    /// asdf current <name>                     Display current version set or being
    ///                                         used for package
    /// ```
    pub fn current<A>(name: Option<A>) -> !
    where
        A: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf where <name> [<version>]           Display install path for an installed
    ///                                         or current version
    /// ```
    pub fn locate<A, B>(name: A, version: Option<B>) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf which <command>                    Display the path to an executable
    /// ```
    pub fn which<A>(command: A) -> !
    where
        A: AsRef<str>,
    {
        todo!();
    }

    /// ```
    /// asdf local <name> <version>             Set the package local version
    /// asdf local <name> latest[:<version>]    Set the package local version to the
    ///                                         latest provided version
    /// ```
    pub fn local<A, B>(name: A, version: B) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf global <name> <version>            Set the package global version
    /// asdf global <name> latest[:<version>]   Set the package global version to the
    ///                                         latest provided version
    /// ```
    pub fn global<A, B>(name: A, version: B) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf shell <name> <version>             Set the package version to
    ///                                         `ASDF_${LANG}_VERSION` in the current shell
    /// ```
    pub fn shell<A, B>(name: A, version: B) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf latest <name> [<version>]          Show latest stable version of a package
    /// ```
    pub fn latest<A, B>(name: A, version: Option<B>) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf latest --all                       Show latest stable version of all the
    ///                                         packages and if they are installed
    /// ```
    pub fn latest_all() -> ! {
        todo!();
    }

    /// ```help
    /// asdf list <name> [version]              List installed versions of a package and
    ///                                         optionally filter the versions
    /// ```
    pub fn list<A, B>(name: A, version: Option<B>) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf list all <name> [<version>]        List all versions of a package and
    ///                                         optionally filter the returned versions
    /// ```
    pub fn list_all<A, B>(name: A, version: Option<B>) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }
}

/// UTILS
pub mod utils {
    use super::{Response, Result};

    /// ```help
    /// asdf exec <command> [args...]           Executes the command shim for current version
    /// ```
    pub fn exec<A, B, C>(command: A, args: C) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
        C: IntoIterator<Item = B>,
    {
        todo!();
    }

    /// ```help
    /// asdf env <command> [util]               Runs util (default: `env`) inside the
    ///                                         environment used for command shim execution.
    /// ```
    pub fn env<A, B>(command: A, util: B) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf info                               Print OS, Shell and ASDF debug information.
    /// ```
    pub fn info() -> ! {
        todo!();
    }

    /// ```help
    /// asdf reshim <name> <version>            Recreate shims for version of a package
    /// ```
    pub fn reshim<A, B>(name: A, version: B) -> !
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf shim-versions <command>            List the plugins and versions that
    ///                                         provide a command
    /// ```
    pub fn shim_versions<A>(name: A) -> !
    where
        A: AsRef<str>,
    {
        todo!();
    }

    /// ```help
    /// asdf update                             Update asdf to the latest stable release
    /// asdf update --head                      Update asdf to the latest on the master branch
    /// ```
    pub fn update(head: bool) -> ! {
        todo!();
    }

    /// Originally in the 'MANAGE PACKAGES' section, moved because that seems wrong...
    ///
    /// ```help
    /// asdf help <name> [<version>]            Output documentation for plugin and tool
    /// ```
    pub fn help() -> ! {
        todo!();
    }
}
