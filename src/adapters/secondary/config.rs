//! Configuration adapter
//!
//! Secondary adapter for loading configuration.

use crate::domain::{ConfigLoader, Result, DomainError};

pub struct JsonConfigLoader {
    data: serde_json::Value,
}

impl JsonConfigLoader {
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }

    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| DomainError::ConfigError(e.to_string()))?;
        Ok(Self { data })
    }
}

impl ConfigLoader for JsonConfigLoader {
    fn load(&self) -> Result<serde_json::Value> {
        Ok(self.data.clone())
    }

    fn get(&self, key: &str) -> Result<Option<serde_json::Value>> {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &self.data;

        for part in parts {
            if let Some(obj) = current.get(part) {
                current = obj;
            } else {
                return Ok(None);
            }
        }

        Ok(Some(current.clone()))
    }
}

pub struct TomlConfigLoader {
    data: toml::Value,
}

impl TomlConfigLoader {
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let data: toml::Value = toml::from_str(&content)
            .map_err(|e| DomainError::ConfigError(e.to_string()))?;
        Ok(Self { data })
    }
}

impl ConfigLoader for TomlConfigLoader {
    fn load(&self) -> Result<serde_json::Value> {
        let json = serde_json::to_value(&self.data)?;
        Ok(json)
    }

    fn get(&self, key: &str) -> Result<Option<serde_json::Value>> {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current: &toml::Value = &self.data;

        for part in parts {
            if let Some(next) = current.get(part) {
                current = next;
            } else {
                return Ok(None);
            }
        }

        let json = serde_json::to_value(current)?;
        Ok(Some(json))
    }
}

