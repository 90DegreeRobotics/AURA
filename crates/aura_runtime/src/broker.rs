//! Action broker — sole executor after Sentinel authorizes.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::{build_guard_request, AuraAction, AuraError, AuraResult, AuraSentinelClient};

pub struct EffectRequest {
    pub action: AuraAction,
    pub resource: Option<String>,
    pub actor_id: Uuid,
    pub declared_intent: String,
    pub payload_hash: String,
    /// Invoked only after Core authorizes effect. Must be pure of pre-auth work.
    pub side_effect: Box<dyn FnOnce() -> AuraResult<Value> + Send>,
}

#[derive(Debug, Clone)]
pub struct EffectOutcome {
    pub action: String,
    pub resource: String,
    pub authorized: bool,
    pub result: Value,
}

#[derive(Debug)]
pub struct ActionBroker {
    client: Arc<AuraSentinelClient>,
    effects_executed: AtomicU64,
}

impl ActionBroker {
    pub fn new(client: Arc<AuraSentinelClient>) -> Self {
        Self {
            client,
            effects_executed: AtomicU64::new(0),
        }
    }

    pub fn effects_executed(&self) -> u64 {
        self.effects_executed.load(Ordering::SeqCst)
    }

    /// Authorize then maybe execute. Deny / shadow / seal-fail ⇒ no side effect.
    pub fn execute(&self, req: EffectRequest) -> AuraResult<EffectOutcome> {
        if !self.client.is_ready() {
            return Err(AuraError::SentinelNotReady(
                "broker will not run without Sentinel",
            ));
        }

        let guard_req = build_guard_request(
            req.action,
            req.resource.as_deref(),
            req.actor_id,
            &req.declared_intent,
            &req.payload_hash,
        )?;

        let decision = self.client.authorize(&guard_req)?;

        if !decision.authorizes_effect() {
            return Err(AuraError::Denied(decision.rationale));
        }

        // Enforce-only execution: even if a future client forgot to strip shadow,
        // refuse when mode is not enforce.
        if !self.client.mode().enforced() {
            return Err(AuraError::Denied(
                "shadow mode cannot execute effects".into(),
            ));
        }

        let result = (req.side_effect)()?;
        self.effects_executed.fetch_add(1, Ordering::SeqCst);

        Ok(EffectOutcome {
            action: guard_req.action,
            resource: guard_req.resource,
            authorized: true,
            result,
        })
    }
}
