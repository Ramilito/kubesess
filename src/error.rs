use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to set context: {0}")]
    SetContext(#[source] SetContextError),
    #[error("no item selected when prompted to select {prompt}")]
    NoItemSelected { prompt: &'static str },
}

// #[derive(Error, Debug)]
// pub enum SetNamespaceError {
//     #[error("no namespace exists with the name {ns}")]
//     KubeNamespaceNotFound { ns: String },
// }

#[derive(Error, Debug)]
pub enum SetContextError {
    #[error("no context exists with the name {ctx}")]
    KubeContextNotFound { ctx: String },
}
