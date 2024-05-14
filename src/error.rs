use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
}

unsafe impl Send for SdkError {}
unsafe impl Sync for SdkError {}