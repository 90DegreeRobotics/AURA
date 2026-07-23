//! Map Aura intent onto protected-action IDs.

use chrono::Utc;
use uuid::Uuid;

use crate::{AuraError, AuraResult, SentinelGuardRequest, AURA_ACTOR_CLASS, AURA_SUBJECT_SYSTEM};

/// Aura-facing action vocabulary. Each variant maps to a Core registry ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuraAction {
    /// Transition from initializing into work mode.
    BootContinue,
    /// Brokered generic effect (must still be Core-known).
    EffectExecute,
    ModelGenerate,
    ToolInvoke,
    MemoryWrite,
    FileWrite,
    NetworkEgress,
    ProcessSpawn,
}

impl AuraAction {
    /// Core protected-action string. Unknown/custom IDs are forbidden here.
    pub fn core_action(self) -> &'static str {
        match self {
            Self::BootContinue | Self::EffectExecute => "effect.execute",
            Self::ModelGenerate => "model.generate",
            Self::ToolInvoke => "tool.invoke",
            Self::MemoryWrite => "memory.write",
            Self::FileWrite => "file.write",
            Self::NetworkEgress => "network.egress",
            Self::ProcessSpawn => "process.spawn",
        }
    }

    pub fn default_resource(self) -> &'static str {
        match self {
            Self::BootContinue => "aura://boot/continue",
            Self::EffectExecute => "aura://effect",
            Self::ModelGenerate => "aura://model",
            Self::ToolInvoke => "aura://tool",
            Self::MemoryWrite => "aura://memory",
            Self::FileWrite => "aura://file",
            Self::NetworkEgress => "aura://network",
            Self::ProcessSpawn => "aura://process",
        }
    }

    pub fn irreversible(self) -> bool {
        matches!(
            self,
            Self::MemoryWrite | Self::FileWrite | Self::ProcessSpawn | Self::NetworkEgress
        )
    }

    pub fn external_impact(self) -> bool {
        matches!(self, Self::NetworkEgress | Self::ToolInvoke)
    }
}

pub fn build_guard_request(
    action: AuraAction,
    resource: Option<&str>,
    actor_id: Uuid,
    declared_intent: &str,
    payload_hash: &str,
) -> AuraResult<SentinelGuardRequest> {
    if declared_intent.trim().is_empty() {
        return Err(AuraError::InvalidRequest(
            "declared_intent must not be empty".into(),
        ));
    }
    if payload_hash.trim().is_empty() {
        return Err(AuraError::InvalidRequest(
            "payload_hash must not be empty".into(),
        ));
    }

    let resource = resource.unwrap_or_else(|| action.default_resource());
    let nonce = Uuid::new_v4();
    let envelope_digest =
        format!("sha256:aura:{action:?}:{resource}:{actor_id}:{nonce}:{payload_hash}");

    Ok(SentinelGuardRequest {
        envelope_version: "sentinel.guard.v1".to_string(),
        action: action.core_action().to_string(),
        resource: resource.to_string(),
        actor_id,
        actor_class: AURA_ACTOR_CLASS.to_string(),
        subject_system: AURA_SUBJECT_SYSTEM.to_string(),
        request_origin: "aura.runtime".to_string(),
        timestamp_utc: Utc::now(),
        nonce,
        payload_hash: payload_hash.to_string(),
        context_digest: "sha256:aura-context-v0".to_string(),
        requested_capability: None,
        consent_reference: None,
        declared_intent: declared_intent.to_string(),
        irreversible_side_effect: action.irreversible(),
        external_impact: action.external_impact(),
        envelope_digest,
    })
}
