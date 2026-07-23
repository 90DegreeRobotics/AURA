//! Sentinel client — the only decision gateway Aura may use.
//!
//! Binds to `sentinel_core::DeterministicSentinelGuard`. Default mode is
//! **enforce**. Shadow is an explicit opt-down and never authorizes effects
//! for broker execution in this crate (shadow may observe, but broker still
//! refuses unless `authorizes_effect()` — and we force deny-all unless enforce
//! path with an authorizing decision).

use std::sync::Arc;

use sentinel_core::{
    DeterministicSentinelGuard, GuardPolicy, SentinelGuard, SentinelGuardDecision,
    SentinelGuardRequest,
};

use crate::{AuraError, AuraResult, DecisionLog};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentinelMode {
    /// Fail closed. Only authorizing decisions may proceed to the broker.
    Enforce,
    /// Observe-only opt-down. Broker must still refuse effects (no teeth for ship).
    Shadow,
}

impl SentinelMode {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "enforce" => Some(Self::Enforce),
            "shadow" => Some(Self::Shadow),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enforce => "enforce",
            Self::Shadow => "shadow",
        }
    }

    pub fn enforced(self) -> bool {
        matches!(self, Self::Enforce)
    }
}

#[derive(Debug)]
pub struct AuraSentinelClient {
    mode: SentinelMode,
    guard: DeterministicSentinelGuard,
    log: Arc<DecisionLog>,
    ready: bool,
}

impl AuraSentinelClient {
    /// Construct the runtime Sentinel. Default policy is **deny-all**.
    pub fn new_deny_all(mode: SentinelMode, log: Arc<DecisionLog>) -> Self {
        let guard = DeterministicSentinelGuard::new(GuardPolicy::deny_all(
            "aura-constitutional-deny-all",
            "0.1.0",
        ));
        Self {
            mode,
            guard,
            log,
            ready: true,
        }
    }

    /// Construct with an explicit Core policy (tests / future signed policy load).
    pub fn with_policy(mode: SentinelMode, policy: GuardPolicy, log: Arc<DecisionLog>) -> Self {
        Self {
            mode,
            guard: DeterministicSentinelGuard::new(policy),
            log,
            ready: true,
        }
    }

    pub fn mode(&self) -> SentinelMode {
        self.mode
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn decision_log(&self) -> &DecisionLog {
        &self.log
    }

    /// Authorize a protected action. Always consults Core. Always seals.
    /// Seal failure ⇒ error (fail closed). Shadow mode never returns an
    /// authorizing decision to the broker (effects stay blocked).
    pub fn authorize(&self, request: &SentinelGuardRequest) -> AuraResult<SentinelGuardDecision> {
        if !self.ready {
            return Err(AuraError::SentinelNotReady(
                "Sentinel client marked not ready",
            ));
        }

        let mut decision = self.guard.authorize(request);

        // Shadow: observe and seal, but strip effect authorization so the
        // broker cannot execute. This keeps shadow from becoming a bypass.
        if matches!(self.mode, SentinelMode::Shadow) && decision.authorizes_effect() {
            decision.allowed = false;
            decision.rationale = format!(
                "shadow mode: observed would-allow but effect blocked ({})",
                decision.rationale
            );
        }

        self.log.append(
            &request.action,
            &request.resource,
            request.actor_id,
            &decision,
        )?;

        Ok(decision)
    }
}
