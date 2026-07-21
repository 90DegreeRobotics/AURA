//! Boot supervisor — no work mode before Sentinel is ready and boot is authorized.

use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    ActionBroker, AuraAction, AuraError, AuraResult, AuraSentinelClient, DecisionLog,
    EffectRequest, SentinelMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BootPhase {
    /// Process started; only preboot journal / status allowed.
    Preboot,
    /// Sentinel client constructed; still not in work mode.
    Initializing,
    /// Boot-continue authorized; protected handlers may be offered.
    Ready,
    /// Sentinel missing / ledger broken / lockdown.
    Blocked,
}

#[derive(Debug, Clone, Serialize)]
pub struct BootStatus {
    pub phase: BootPhase,
    pub sentinel_mode: String,
    pub sentinel_ready: bool,
    pub enforced: bool,
    pub message: String,
}

/// Owns boot order. Callers cannot jump to Ready without authorize.
pub struct BootSupervisor {
    phase: BootPhase,
    client: Arc<AuraSentinelClient>,
    broker: ActionBroker,
    actor_id: Uuid,
}

impl BootSupervisor {
    /// Construct Aura with deny-all Sentinel in enforce mode by default.
    pub fn start_enforce(log: Arc<DecisionLog>) -> Self {
        Self::start(SentinelMode::Enforce, log)
    }

    pub fn start(mode: SentinelMode, log: Arc<DecisionLog>) -> Self {
        let client = Arc::new(AuraSentinelClient::new_deny_all(mode, log));
        let broker = ActionBroker::new(Arc::clone(&client));
        Self {
            phase: BootPhase::Initializing,
            client,
            broker,
            actor_id: Uuid::new_v4(),
        }
    }

    pub fn with_client(client: Arc<AuraSentinelClient>, actor_id: Uuid) -> Self {
        let phase = if client.is_ready() {
            BootPhase::Initializing
        } else {
            BootPhase::Blocked
        };
        let broker = ActionBroker::new(Arc::clone(&client));
        Self {
            phase,
            client,
            broker,
            actor_id,
        }
    }

    pub fn phase(&self) -> BootPhase {
        self.phase
    }

    pub fn client(&self) -> &AuraSentinelClient {
        &self.client
    }

    pub fn broker(&self) -> &ActionBroker {
        &self.broker
    }

    pub fn actor_id(&self) -> Uuid {
        self.actor_id
    }

    pub fn status(&self) -> BootStatus {
        let ready = self.client.is_ready();
        let message = match self.phase {
            BootPhase::Preboot => "preboot only — Sentinel not constructed".into(),
            BootPhase::Initializing => {
                "Sentinel ready; work mode blocked until boot-continue authorized".into()
            }
            BootPhase::Ready => "work mode open under Sentinel enforce path".into(),
            BootPhase::Blocked => "blocked — Sentinel unavailable or lockdown".into(),
        };
        BootStatus {
            phase: self.phase,
            sentinel_mode: self.client.mode().as_str().to_string(),
            sentinel_ready: ready,
            enforced: self.client.mode().enforced(),
            message,
        }
    }

    /// Attempt to enter work mode via brokered `effect.execute` on boot resource.
    /// Under deny-all this always fails closed and phase stays Initializing.
    pub fn try_continue_boot(&mut self) -> AuraResult<BootStatus> {
        if !self.client.is_ready() {
            self.phase = BootPhase::Blocked;
            return Err(AuraError::SentinelNotReady("cannot continue boot"));
        }
        if matches!(self.phase, BootPhase::Ready) {
            return Ok(self.status());
        }

        let actor_id = self.actor_id;
        match self.broker.execute(EffectRequest {
            action: AuraAction::BootContinue,
            resource: None,
            actor_id,
            declared_intent: "enter Aura work mode after Sentinel ready".into(),
            payload_hash: "sha256:aura-boot-continue".into(),
            side_effect: Box::new(|| Ok(json!({"boot": "continued"}))),
        }) {
            Ok(_) => {
                self.phase = BootPhase::Ready;
                Ok(self.status())
            }
            Err(e) => Err(e),
        }
    }
}
