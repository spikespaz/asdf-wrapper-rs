use std::{
    ffi::OsStr,
    process::{Command, Output},
};
// use strum;
use getset::Getters;
use thiserror::Error;

/// This enumerable represents all possible [`Err`] values of the [`Result`] alias used by this library.
///
#[derive(Debug, Error)]
pub enum Error {
    /// When a call to [`std::process::Command::output`] fails to run a command,
    /// the [`std::io::Error`] will be wrapped unless a specific case is handled by another variant.
    #[error("unable to create command")]
    IoError(std::io::Error),
    /// When standard output and error from a call to [`std::process::Command::output`] do not
    /// convert to valid unicode.
    #[error("failed to get command output as UTF-8")]
    BadEncoding(#[from] std::string::FromUtf8Error),
    /// This variant will be used after checking the command output against all cases that warrant other
    /// variants, and if the [`std::process::Output::status`] is non-zero.
    #[error("command terminated unsuccessfully")]
    FailedCommand(Command),
    /// Instead of returning [`Error::IoError`] when the operating system
    /// reports code `2` ([`std::io::ErrorKind::NotFound`]), this variant is used.
    #[error("asdf command was not found on this system")]
    AsdfNotFound,
    /// An parameter to a function (argument to a command) was incorrect. Generally this is caused by spaces
    /// in a string passed as a parameter, though the shell or Rust should escape these for you.
    /// It is more likely that there were illegal characters passed to `asdf`.
    #[error("the parameters passed were incorrect or malformed")]
    MalformedOptions(Command),
    /// This is an error within the library. Perhaps an update to `asdf` broke this, and the output or
    /// formatting of a certain command has changed. Please notify the mainainer of this library if this
    /// variant is passed to a caller.
    #[error("the output from the command was unexpected")]
    MalformedOutput(Command),
    /// An `asdf` command needed to access the internet to do something and the system had no connection.
    #[error("there is no internet connection")]
    NoInternet,
    /// The plugin name that was provided as an argument to an `asdf` command
    /// was not installed or does not exist in the repository.
    #[error("the plugin requested was not found in the repository")]
    PluginNotFound,
    /// The plugin that you requested to install is already available on the system.
    #[error("the plugin requested was already added")]
    PluginAlreadyAdded,
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        match other.kind() {
            std::io::ErrorKind::NotFound => Error::AsdfNotFound,
            _ => Error::IoError(other),
        }
    }
}

/// An alias of [`std::result::Result`] specific to this library.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct CommandResult {
    command: Command,
    output: Output,
    stdout: String,
    stderr: String,
}

fn command<S, I>(run: I) -> Result<CommandResult>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    let mut run = run.into_iter();
    let mut command = Command::new(run.next().unwrap());

    command.args(run);

    let output = command.output()?;
    let stdout = String::from_utf8(output.stdout.clone())?;
    let stderr = String::from_utf8(output.stderr.clone())?;

    Ok(CommandResult {
        command,
        output,
        stdout,
        stderr,
    })
}

/// MANAGE PLUGINS
pub mod plugins {
    use super::*;

    /// Structure representing an `asdf` plugin as returned from [`list`] or [`list_all`].
    #[derive(Clone, Debug, PartialEq, Getters)]
    #[getset(get)]
    pub struct Plugin {
        /// Name of the plugin as specified in the plugin repository, or by the argument when
        /// installed from a Git clone URL.
        name: String,
        /// The Git clone URL for this plugin.
        git_url: String,
        /// The Git branch that is used for the installed version of this plugin.
        git_branch: Option<String>,
        /// The Git reference that is used for the installed version of this plugin.
        git_ref: Option<String>,
    }

    pub type PluginSet = Vec<Plugin>;

    impl TryFrom<&str> for Plugin {
        type Error = &'static str;

        /// Given a [`&str`], this function splits at whitespace and places the results into the fields of this struct
        /// in the order they are defined. The first two, `name` and `git_url` are mandatory, whereas
        /// `git_branch` and `git_ref` are assumed to be [`None`] if the [`std::str::SplitWhitespace`]
        /// iterator ends after `git_url`.
        fn try_from(other: &str) -> std::result::Result<Self, Self::Error> {
            let mut parts = other.split_whitespace();

            Ok(Self {
                name: match parts.next() {
                    Some(name) => name.to_owned(),
                    None => return Err("not enough iterations to retrieve field `name`"),
                },
                git_url: match parts.next() {
                    Some(name) => name.to_owned(),
                    None => return Err("not enough iterations to retrieve field `git_url`"),
                },
                git_branch: parts.next().map(|x| x.to_owned()),
                git_ref: parts.next().map(|x| x.to_owned()),
            })
        }
    }

    /// Add a plugin by name from the repository, or with a name by a valid Git clone URL.
    /// See the [`asdf` plugin repository](https://github.com/asdf-vm/asdf-plugins#plugin-list)
    /// for a complete list, or use `asdf plugin list all` or [`list_all`].
    ///
    /// ```help
    /// asdf plugin add <name> [<git-url>]      Add a plugin from the plugin repo OR,
    ///                                         add a Git repo as a plugin by
    ///                                         specifying the name and repo url
    /// ```
    pub fn add<A, B>(name: A, git_url: Option<B>) -> Result<()>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let mut run = vec!["asdf", "plugin", "list", name.as_ref()];

        if let Some(git_url) = &git_url {
            run.extend([git_url.as_ref()]);
        }

        let CommandResult {
            command,
            output,
            stdout: _,
            stderr,
        } = command(run)?;

        if stderr.contains("not found in repository") {
            Err(Error::PluginNotFound)
        } else if stderr.contains("already added") {
            Err(Error::PluginAlreadyAdded)
        } else if stderr.contains("usage: asdf") {
            Err(Error::MalformedOptions(command))
        } else if output.status.success() {
            Ok(())
        } else {
            Err(Error::FailedCommand(command))
        }
    }

    /// Returns a [`PluginSet`] of [`Plugin`] for all plugins installed.
    /// To get an iterable of available plugins, see [`list_all`].
    /// Unlike the command, this function will always return a [`PluginSet`] with output from `--urls` and `--refs`.
    ///
    /// ```help
    /// asdf plugin list [--urls] [--refs]      List installed plugins. Optionally show
    ///                                         git urls and git-ref
    /// ```
    pub fn list() -> Result<PluginSet> {
        let CommandResult {
            command,
            output,
            stdout,
            stderr,
        } = command(["asdf", "plugin", "list", "--urls", "--refs"])?;

        if stderr.contains("No plugins installed") {
            Ok(PluginSet::new())
        } else if output.status.success() {
            stdout
                .lines()
                .map(|line| line.try_into())
                .collect::<std::result::Result<PluginSet, _>>()
                .or(Err(Error::MalformedOutput(command)))
        } else {
            Err(Error::FailedCommand(command))
        }
    }

    /// ```help
    /// asdf plugin list all                    List plugins registered on asdf-plugins
    ///                                         repository with URLs
    /// ```
    pub fn list_all() -> Result<PluginSet> {
        let CommandResult {
            command,
            output,
            stdout,
            stderr,
        } = command(["asdf", "plugin", "list", "all"])?;

        if stderr.contains("Could not resolve host") {
            Err(Error::NoInternet)
        } else if output.status.success() {
            stdout
                .replace("initializing plugin repository...", "")
                .lines()
                .map(|line| line.try_into())
                .collect::<std::result::Result<PluginSet, _>>()
                .or(Err(Error::MalformedOutput(command)))
        } else {
            Err(Error::FailedCommand(command))
        }
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
    use super::*;

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

    /// ```help
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
    use super::*;

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
