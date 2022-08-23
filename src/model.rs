use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Contexts {
    pub context: Context,
    pub name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub namespace: String,
    pub cluster: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub kind: String,
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub api_version: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "current-context")]
    pub current_context: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub contexts: Vec<Contexts>,
}
