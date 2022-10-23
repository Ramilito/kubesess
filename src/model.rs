use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Contexts {
    #[serde(default)]
    pub context: Context,
    #[serde(default)]
    pub name: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub namespace: String,
    pub cluster: String,
    pub user: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub kind: String,
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub api_version: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "current-context")]
    pub current_context: String,
    #[serde(default)]
    pub contexts: Vec<Contexts>,
}
