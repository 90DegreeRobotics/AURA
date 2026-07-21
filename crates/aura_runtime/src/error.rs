use thiserror::Error;

pub type AuraResult<T> = Result<T, AuraError>;

#[derive(Debug, Error)]
pub enum AuraError {
    #[error("Sentinel is not ready: {0}")]
    SentinelNotReady(&'static str),

    #[error("boot refused: {0}")]
    BootRefused(String),

    #[error("Sentinel denied effect: {0}")]
    Denied(String),

    #[error("decision ledger append failed: {0}")]
    LedgerFailed(String),

    #[error("effect refused: authorization did not authorize effect")]
    EffectNotAuthorized,

    #[error("invalid request: {0}")]
    InvalidRequest(String),
}
