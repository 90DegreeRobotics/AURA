//! Append-only decision evidence for Aura (local seal until Core ledger wiring).
//!
//! Forever Law: if the seal fails, the action must fail. Broker refuses when
//! append fails.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use uuid::Uuid;

use crate::{AuraError, AuraResult, SentinelGuardDecision};

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

#[derive(Debug, Serialize)]
struct DecisionRecordSeal<'a> {
    seq: u64,
    recorded_at_utc: DateTime<Utc>,
    action: &'a str,
    resource: &'a str,
    actor_id: Uuid,
    allowed: bool,
    authorizes_effect: bool,
    class: &'a str,
    rationale: &'a str,
    policy_id: &'a str,
    policy_version: &'a str,
    trace_id: Uuid,
    prev_hash: &'a str,
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
                tip_hash: decision_genesis_hash(),
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
        let seal = DecisionRecordSeal {
            seq,
            action,
            resource,
            actor_id,
            recorded_at_utc,
            allowed: decision.allowed,
            authorizes_effect: decision.authorizes_effect(),
            class: &class,
            rationale: &decision.rationale,
            policy_id: &decision.policy_id,
            policy_version: &decision.policy_version,
            trace_id: decision.trace_id,
            prev_hash: &prev_hash,
        };
        let record_hash = blake3_with_prefix(
            &serde_json::to_vec(&seal)
                .map_err(|e| AuraError::LedgerFailed(format!("serialize decision seal: {e}")))?,
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

fn decision_genesis_hash() -> String {
    blake3_with_prefix(b"neurocognica.aura.decision_log.genesis.v1")
}

fn blake3_with_prefix(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GuardDecisionClass;

    #[test]
    fn decision_log_uses_real_blake3_hash_chain() {
        let path = temp_log_path("decision-hash-chain");
        let log = DecisionLog::open(&path).expect("log opens");
        let actor_id = Uuid::new_v4();
        let decision = SentinelGuardDecision {
            trace_id: Uuid::new_v4(),
            class: GuardDecisionClass::Deny,
            allowed: false,
            monitoring_required: false,
            rationale: "unit test denial".to_owned(),
            policy_id: "test-policy".to_owned(),
            policy_version: "test".to_owned(),
            matched_rule_id: None,
            violations: Vec::new(),
            ledger_event_hash: None,
        };

        let first = log
            .append("memory.write", "aura://memory/test", actor_id, &decision)
            .expect("first record appends");
        let second = log
            .append("memory.write", "aura://memory/test", actor_id, &decision)
            .expect("second record appends");

        assert!(first.prev_hash.starts_with("blake3:"));
        assert!(first.record_hash.starts_with("blake3:"));
        assert_eq!(second.prev_hash, first.record_hash);
        assert_ne!(second.record_hash, first.record_hash);
    }

    fn temp_log_path(stem: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "aura-runtime-{stem}-{}-{nanos}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("temp root");
        root.join("decisions.jsonl")
    }
}
