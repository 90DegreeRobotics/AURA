//! Append-only decision evidence for Aura (local seal until Core ledger wiring).
//!
//! Forever Law: if the seal fails, the action must fail. Broker refuses when
//! append fails.

use chrono::{DateTime, Utc};
use sentinel_core::SentinelGuardDecision;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use uuid::Uuid;

use crate::{AuraError, AuraResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecisionRecord {
    pub seq: u64,
    pub recorded_at_utc: DateTime<Utc>,
    pub action: String,
    pub resource: String,
    pub actor_id: Uuid,
    pub allowed: bool,
    pub authorizes_effect: bool,
    pub class: String,
    pub rationale: String,
    pub policy_id: String,
    pub policy_version: String,
    pub trace_id: Uuid,
    pub prev_hash: String,
    pub record_hash: String,
}

#[derive(Debug)]
pub struct DecisionLog {
    path: PathBuf,
    inner: Mutex<LogState>,
}

#[derive(Debug)]
struct LogState {
    seq: u64,
    tip_hash: String,
}

impl DecisionLog {
    pub fn open(path: impl Into<PathBuf>) -> AuraResult<Self> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AuraError::LedgerFailed(format!("create decision log dir: {e}")))?;
        }
        // Touch file so absence is not silent.
        if !path.exists() {
            File::create(&path)
                .map_err(|e| AuraError::LedgerFailed(format!("create decision log: {e}")))?;
        }
        Ok(Self {
            path,
            inner: Mutex::new(LogState {
                seq: 0,
                tip_hash: "sha256:genesis".to_string(),
            }),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append(
        &self,
        action: &str,
        resource: &str,
        actor_id: Uuid,
        decision: &SentinelGuardDecision,
    ) -> AuraResult<DecisionRecord> {
        let mut state = self
            .inner
            .lock()
            .map_err(|_| AuraError::LedgerFailed("decision log mutex poisoned".into()))?;

        let seq = state
            .seq
            .checked_add(1)
            .ok_or_else(|| AuraError::LedgerFailed("decision log sequence overflow".into()))?;
        let recorded_at_utc = Utc::now();
        let class = format!("{:?}", decision.class);
        let prev_hash = state.tip_hash.clone();
        let record_hash = format!(
            "sha256:{}:{}:{}:{}:{}:{}:{}",
            seq,
            action,
            resource,
            decision.allowed,
            decision.trace_id,
            prev_hash,
            recorded_at_utc.timestamp_nanos_opt().unwrap_or(0)
        );

        let record = DecisionRecord {
            seq,
            recorded_at_utc,
            action: action.to_string(),
            resource: resource.to_string(),
            actor_id,
            allowed: decision.allowed,
            authorizes_effect: decision.authorizes_effect(),
            class,
            rationale: decision.rationale.clone(),
            policy_id: decision.policy_id.clone(),
            policy_version: decision.policy_version.clone(),
            trace_id: decision.trace_id,
            prev_hash,
            record_hash: record_hash.clone(),
        };

        let line = serde_json::to_string(&record)
            .map_err(|e| AuraError::LedgerFailed(format!("serialize decision: {e}")))?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| AuraError::LedgerFailed(format!("open decision log: {e}")))?;
        writeln!(file, "{line}")
            .map_err(|e| AuraError::LedgerFailed(format!("append decision log: {e}")))?;
        file.flush()
            .map_err(|e| AuraError::LedgerFailed(format!("flush decision log: {e}")))?;

        state.seq = seq;
        state.tip_hash = record_hash;
        Ok(record)
    }
}
