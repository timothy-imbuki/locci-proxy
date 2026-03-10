use thiserror::Error;
use pingora::Error as PingoraError;

pub type ProxyResult<T> = Result<T, ProxyError>;

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Address parse error: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Invalid regex pattern '{pattern}': {source}")]
    InvalidRegex { pattern: String, source: regex::Error },
    #[error("Pingora error: {0}")]
    Pingora(#[from] Box<PingoraError>),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Upstream '{0}' not found")]
    UpstreamNotFound(String),
    #[error("No servers in upstream '{0}'")]
    EmptyUpstream(String),
}
