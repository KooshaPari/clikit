//! Plugin system - Extensibility
//!
//! Plugins allow extending clikit with custom commands and functionality.

use crate::domain::{Command, DomainError, Plugin, Result};
use libloading::{Library, Symbol};
use std::path::Path;

pub struct PluginManager {
    plugins: Vec<LoadedPlugin>,
}

struct LoadedPlugin {
    name: String,
    library: Library,
    _plugin: Box<dyn Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_from_dir(&mut self, dir: &Path) -> Result<Vec<String>> {
        if !dir.exists() {
            return Err(DomainError::PluginError(format!(
                "Plugin directory does not exist: {:?}",
                dir
            )));
        }

        let mut loaded = Vec::new();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path
                .extension()
                .map(|e| e == "so" || e == "dll" || e == "dylib")
                .unwrap_or(false)
            {
                if let Ok(name) = self.load_plugin(&path) {
                    loaded.push(name);
                }
            }
        }

        Ok(loaded)
    }

    pub fn load_plugin(&mut self, path: &Path) -> Result<String> {
        #[cfg(unix)]
        let lib =
            unsafe { Library::new(path) }.map_err(|e| DomainError::PluginError(e.to_string()))?;

        #[cfg(windows)]
        let lib =
            unsafe { Library::new(path) }.map_err(|e| DomainError::PluginError(e.to_string()))?;

        type CreatePlugin = fn() -> Box<dyn Plugin>;

        let create: Symbol<CreatePlugin> = unsafe { lib.get(b"create_plugin") }.map_err(|e| {
            DomainError::PluginError(format!("Failed to get create_plugin: {:?}", e))
        })?;

        let plugin = create();
        let name = plugin.name().to_string();

        plugin.init()?;

        self.plugins.push(LoadedPlugin {
            name: name.clone(),
            library: lib,
            _plugin: plugin,
        });

        Ok(name)
    }

    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name.as_str()).collect()
    }

    pub fn get_commands(&self) -> Vec<Command> {
        self.plugins
            .iter()
            .flat_map(|p| p._plugin.commands())
            .collect()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

// Plugin must be implemented by plugins
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    // This function must be implemented by each plugin
    // For now, return a placeholder
    unimplemented!("Plugin must implement create_plugin()")
}
