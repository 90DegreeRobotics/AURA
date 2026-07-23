//! Handler-level proof that Aura fails closed before side effects.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use uuid::Uuid;

use aura_runtime::{
    ActionBroker, AuraAction, AuraSentinelClient, BootPhase, BootSupervisor, DecisionLog,
    EffectRequest, GuardPolicy, GuardRule, SentinelMode,
};

fn temp_log(name: &str) -> Arc<DecisionLog> {
    let dir = std::env::temp_dir().join("aura_runtime_tests");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{name}-{}.jsonl", Uuid::new_v4()));
    Arc::new(DecisionLog::open(path).unwrap())
}

#[test]
fn default_boot_is_initializing_under_enforce_deny_all() {
    let boot = BootSupervisor::start_enforce(temp_log("boot-init"));
    let status = boot.status();
    assert_eq!(boot.phase(), BootPhase::Initializing);
    assert!(status.enforced);
    assert_eq!(status.sentinel_mode, "enforce");
    assert!(status.sentinel_ready);
}

#[test]
fn deny_all_blocks_boot_continue_and_executes_no_effect() {
    let mut boot = BootSupervisor::start_enforce(temp_log("boot-deny"));
    let before = boot.broker().effects_executed();
    let err = boot
        .try_continue_boot()
        .expect_err("deny-all must refuse boot");
    assert!(
        matches!(err, aura_runtime::AuraError::Denied(_)),
        "expected Denied, got {err:?}"
    );
    assert_eq!(boot.phase(), BootPhase::Initializing);
    assert_eq!(boot.broker().effects_executed(), before);
}

#[test]
fn deny_all_paralyzes_model_generate_before_side_effect() {
    let log = temp_log("model-deny");
    let client = Arc::new(AuraSentinelClient::new_deny_all(SentinelMode::Enforce, log));
    let broker = ActionBroker::new(Arc::clone(&client));
    let fired = Arc::new(AtomicBool::new(false));
    let fired2 = Arc::clone(&fired);

    let err = broker
        .execute(EffectRequest {
            action: AuraAction::ModelGenerate,
            resource: None,
            actor_id: Uuid::new_v4(),
            declared_intent: "unit test model generate".into(),
            payload_hash: "sha256:test-model".into(),
            side_effect: Box::new(move || {
                fired2.store(true, Ordering::SeqCst);
                Ok(serde_json::json!({"tokens": "should-not-exist"}))
            }),
        })
        .expect_err("deny-all must block model.generate");

    assert!(matches!(err, aura_runtime::AuraError::Denied(_)));
    assert!(!fired.load(Ordering::SeqCst));
    assert_eq!(broker.effects_executed(), 0);
}

#[test]
fn shadow_mode_never_executes_even_with_allow_policy() {
    let log = temp_log("shadow");
    let policy = GuardPolicy::explicit(
        "aura-test-allow",
        "0.1.0",
        vec![GuardRule::allow(
            "allow-effect",
            "effect.execute",
            "aura://boot/continue",
            "aura.runtime",
            "aura",
            "test allow",
        )],
    );
    let client = Arc::new(AuraSentinelClient::with_policy(
        SentinelMode::Shadow,
        policy,
        log,
    ));
    let broker = ActionBroker::new(client);
    let fired = Arc::new(AtomicBool::new(false));
    let fired2 = Arc::clone(&fired);

    let err = broker
        .execute(EffectRequest {
            action: AuraAction::BootContinue,
            resource: None,
            actor_id: Uuid::new_v4(),
            declared_intent: "shadow must not execute".into(),
            payload_hash: "sha256:shadow".into(),
            side_effect: Box::new(move || {
                fired2.store(true, Ordering::SeqCst);
                Ok(serde_json::json!({}))
            }),
        })
        .expect_err("shadow must not execute");

    assert!(matches!(err, aura_runtime::AuraError::Denied(_)));
    assert!(!fired.load(Ordering::SeqCst));
}

#[test]
fn explicit_allow_under_enforce_runs_side_effect_once() {
    let log = temp_log("allow");
    let policy = GuardPolicy::explicit(
        "aura-test-allow",
        "0.1.0",
        vec![GuardRule::allow(
            "allow-model",
            "model.generate",
            "aura://model",
            "aura.runtime",
            "aura",
            "test allow model",
        )],
    );
    let client = Arc::new(AuraSentinelClient::with_policy(
        SentinelMode::Enforce,
        policy,
        log,
    ));
    let broker = ActionBroker::new(client);
    let hits = Arc::new(AtomicU64::new(0));
    let hits2 = Arc::clone(&hits);

    let out = broker
        .execute(EffectRequest {
            action: AuraAction::ModelGenerate,
            resource: None,
            actor_id: Uuid::new_v4(),
            declared_intent: "authorized model generate".into(),
            payload_hash: "sha256:model-ok".into(),
            side_effect: Box::new(move || {
                hits2.fetch_add(1, Ordering::SeqCst);
                Ok(serde_json::json!({"ok": true}))
            }),
        })
        .expect("enforce+allow must run");

    assert!(out.authorized);
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    assert_eq!(broker.effects_executed(), 1);
}

#[test]
fn decision_log_records_denials() {
    let log = temp_log("ledger");
    let path = log.path().to_path_buf();
    let client = Arc::new(AuraSentinelClient::new_deny_all(SentinelMode::Enforce, log));
    let broker = ActionBroker::new(client);
    let _ = broker.execute(EffectRequest {
        action: AuraAction::FileWrite,
        resource: None,
        actor_id: Uuid::new_v4(),
        declared_intent: "ledger denial".into(),
        payload_hash: "sha256:file".into(),
        side_effect: Box::new(|| Ok(serde_json::json!({}))),
    });
    let body = std::fs::read_to_string(path).unwrap();
    assert!(body.contains("file.write"));
    assert!(body.contains("\"allowed\":false"));
}
