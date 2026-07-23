//! Aura L0 runtime — Sentinel first.
//!
//! Carved Law: there is no gate before the Sentinel.
//! Protected effects exist only after AURA's local L0 guard authorizes them.
//! Default policy is deny-all. Missing Sentinel ⇒ no work mode.

mod boot;
mod broker;
mod client;
mod decision_log;
mod error;
mod request;
mod sentinel;

pub use boot::{BootPhase, BootStatus, BootSupervisor};
pub use broker::{ActionBroker, EffectOutcome, EffectRequest};
pub use client::{AuraSentinelClient, SentinelMode};
pub use decision_log::{DecisionLog, DecisionRecord};
pub use error::{AuraError, AuraResult};
pub use request::{build_guard_request, AuraAction};
pub use sentinel::{
    is_protected_action, DeterministicSentinelGuard, GuardDecisionClass, GuardPolicy,
    GuardPolicyMode, GuardRule, SentinelGuard, SentinelGuardDecision, SentinelGuardRequest,
    SentinelGuardViolation, PROTECTED_ACTIONS,
};

/// Product subject system id presented to the local guard.
pub const AURA_SUBJECT_SYSTEM: &str = "aura";

/// Actor class for the Aura runtime itself.
pub const AURA_ACTOR_CLASS: &str = "aura.runtime";
