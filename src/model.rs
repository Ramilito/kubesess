use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Contexts {
    #[serde(default)]
    pub context: Context,
    #[serde(default)]
    pub name: String,
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Clusters {
    #[serde(default)]
    pub cluster: Cluster,
    #[serde(default)]
    pub name: String,
}
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(rename = "certificate-authority-data")]
    pub certificate_authority_data: String,
    pub server: String,
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Users {
    #[serde(default)]
    pub user: HashMap<String, Value>,
    #[serde(default)]
    pub name: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "client-certificate-data")]
    pub client_certificate_data: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "client-key-data")]
    pub client_key_data: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub namespace: String,
    pub cluster: String,
    pub user: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeConfig {
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
    #[serde(default)]
    pub users: Vec<Users>,
    #[serde(default)]
    pub clusters: Vec<Clusters>,
}
