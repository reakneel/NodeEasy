use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

const SERVICE: &str = "nodeeasy";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMaterial {
    pub options: Value,
}

#[derive(Clone, Default)]
pub struct SecretStore;

impl SecretStore {
    pub fn new() -> Self { Self }
    fn entry(node_id: Uuid) -> Result<keyring::Entry, String> {
        keyring::Entry::new(SERVICE, &node_id.to_string()).map_err(|e| e.to_string())
    }
    pub fn set(&self, node_id: Uuid, material: &SecretMaterial) -> Result<(), String> {
        let raw = serde_json::to_string(material).map_err(|e| e.to_string())?;
        Self::entry(node_id)?.set_password(&raw).map_err(|e| e.to_string())
    }
    pub fn get(&self, node_id: Uuid) -> Result<Option<SecretMaterial>, String> {
        match Self::entry(node_id)?.get_password() {
            Ok(raw) => serde_json::from_str(&raw).map(Some).map_err(|e| e.to_string()),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
    pub fn delete(&self, node_id: Uuid) -> Result<(), String> {
        match Self::entry(node_id)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
