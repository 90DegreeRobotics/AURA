use std::path::{Path, PathBuf};

use aura_documents::{default_document_dir, DocumentStore};
use aura_runtime::{BootPhase, BootStatus};

pub const APP_NAME: &str = "AURA";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherSnapshot {
    pub version_line: String,
    pub phase_line: String,
    pub sentinel_line: String,
    pub ledger_line: String,
    pub document_db_line: String,
    pub document_gate_line: String,
    pub effects_line: String,
    pub services_line: String,
    pub message_line: String,
    pub last_event_line: String,
}

impl LauncherSnapshot {
    pub fn from_runtime(
        status: &BootStatus,
        data_dir: &Path,
        ledger_path: &Path,
        effects_executed: u64,
        boot_attempts: u64,
        last_event: &str,
    ) -> Self {
        Self {
            version_line: version_line(),
            phase_line: format!("Boot phase: {}", phase_label(status.phase)),
            sentinel_line: format!(
                "Sentinel: {} | mode {} | enforce {}",
                ready_label(status.sentinel_ready),
                status.sentinel_mode,
                yes_no(status.enforced)
            ),
            ledger_line: format!("Decision ledger: {}", display_path(ledger_path)),
            document_db_line: document_db_line(data_dir),
            document_gate_line: document_gate_line(),
            effects_line: format!(
                "Effects executed: {effects_executed} | boot attempts: {boot_attempts}"
            ),
            services_line:
                "Local services: chat / image / TTS / STT planned, not live in this build"
                    .to_owned(),
            message_line: status.message.clone(),
            last_event_line: format!("Last event: {last_event}"),
        }
    }

    pub fn fatal(ledger_path: &Path, error: &str) -> Self {
        let data_dir = ledger_path.parent().unwrap_or_else(|| Path::new("data"));
        Self {
            version_line: version_line(),
            phase_line: "Boot phase: blocked".to_owned(),
            sentinel_line: "Sentinel: unavailable | mode enforce | enforce yes".to_owned(),
            ledger_line: format!("Decision ledger: {}", display_path(ledger_path)),
            document_db_line: document_db_line(data_dir),
            document_gate_line: document_gate_line(),
            effects_line: "Effects executed: 0 | boot attempts: 0".to_owned(),
            services_line:
                "Local services: chat / image / TTS / STT planned, not live in this build"
                    .to_owned(),
            message_line: "blocked before work mode".to_owned(),
            last_event_line: format!("Last event: runtime refused to start: {error}"),
        }
    }
}

pub fn default_data_dir() -> PathBuf {
    if let Ok(path) = std::env::var("AURA_DATA_DIR") {
        return PathBuf::from(path);
    }
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        return PathBuf::from(local).join("NeuroCognica").join("AURA");
    }
    PathBuf::from("data")
}

pub fn decision_log_path(data_dir: &Path) -> PathBuf {
    data_dir.join("decisions.jsonl")
}

pub fn document_db_line(data_dir: &Path) -> String {
    let document_dir = default_document_dir(data_dir);
    match DocumentStore::summary_at(&document_dir) {
        Ok(summary) => format!(
            "Document DB: {} | framed docs: {} | chunks: {}",
            display_path(&summary.root),
            summary.frame_count,
            summary.chunk_count
        ),
        Err(error) => format!(
            "Document DB: {} | unreadable: {error}",
            display_path(&document_dir)
        ),
    }
}

pub fn document_gate_line() -> String {
    "Document gate: NeuroCognica frame-first store live; embeddings/retrieval/import UI planned"
        .to_owned()
}

pub fn version_line() -> String {
    format!("{APP_NAME} v{VERSION} | build {}", build_profile())
}

fn phase_label(phase: BootPhase) -> &'static str {
    match phase {
        BootPhase::Preboot => "preboot",
        BootPhase::Initializing => "initializing",
        BootPhase::Ready => "ready",
        BootPhase::Blocked => "blocked",
    }
}

fn build_profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

fn ready_label(ready: bool) -> &'static str {
    if ready {
        "ready"
    } else {
        "not ready"
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decision_log_lives_under_data_dir() {
        let path = decision_log_path(Path::new(r"C:\AuraData"));
        assert_eq!(path, PathBuf::from(r"C:\AuraData\decisions.jsonl"));
    }

    #[test]
    fn snapshot_marks_default_runtime_as_initializing() {
        let status = BootStatus {
            phase: BootPhase::Initializing,
            sentinel_mode: "enforce".to_owned(),
            sentinel_ready: true,
            enforced: true,
            message: "Sentinel ready; work mode blocked".to_owned(),
        };
        let snapshot = LauncherSnapshot::from_runtime(
            &status,
            Path::new(r"C:\AuraData"),
            Path::new(r"C:\AuraData\decisions.jsonl"),
            0,
            1,
            "boot refused",
        );
        assert!(snapshot.version_line.starts_with("AURA v"));
        assert_eq!(snapshot.phase_line, "Boot phase: initializing");
        assert!(snapshot.sentinel_line.contains("mode enforce"));
        assert!(snapshot.document_db_line.contains("Document DB:"));
        assert!(snapshot.document_gate_line.contains("frame-first"));
        assert!(snapshot.effects_line.contains("boot attempts: 1"));
        assert!(snapshot.last_event_line.contains("boot refused"));
    }
}
