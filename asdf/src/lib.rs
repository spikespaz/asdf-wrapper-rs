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
