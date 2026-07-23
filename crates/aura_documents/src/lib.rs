//! NeuroCognica document framing for AURA.
//!
//! The old `C:\AURA-Lab\Doc_Framer\nc-framer.py` script framed schematic PNGs with a
//! NeuroCognica title block. This crate preserves and expands that required intake shape for
//! text documents: metadata first, deterministic hashes, stable chunks, a branded print-ready
//! artifact, then storage.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{Datelike, Utc};
use rocksdb::{Options, WriteBatch, WriteOptions, DB};
use serde::{Deserialize, Serialize};

pub const FRAME_SCHEMA_VERSION: &str = "neurocognica.document_frame.v1";
pub const CHUNK_SCHEMA_VERSION: &str = "neurocognica.document_chunk.v1";
pub const PRINT_SCHEMA_VERSION: &str = "neurocognica.document_print.v1";
pub const PRINT_FORMAT_HTML: &str = "html";
pub const PRINT_PAGE_PROFILE: &str = "us_letter_print_v1";
pub const PRINT_LOGO_ASSET_NAME: &str = "neurocognica_logo_transparent.png";
pub const DOCUMENT_STORE_SCHEMA_VERSION: &str = "neurocognica.document_store.rocksdb.v1";
pub const DOCUMENT_FOREVER_SCHEMA_VERSION: &str = "neurocognica.document_forever_event.v1";
pub const DOCUMENT_MMR_SCHEMA_VERSION: &str = "neurocognica.document_mmr.v1";
pub const DOCUMENT_STORE_ENGINE: &str = "rocksdb";
pub const DOCUMENT_ROCKSDB_DIR_NAME: &str = "documents.rocksdb";
pub const DEFAULT_MAX_CHUNK_CHARS: usize = 1_600;
pub const DEFAULT_CHUNK_OVERLAP_CHARS: usize = 160;

const STORE_SCHEMA_KEY: &[u8] = b"meta:store_schema_version";
const FRAME_COUNT_KEY: &[u8] = b"meta:frame_count";
const CHUNK_COUNT_KEY: &[u8] = b"meta:chunk_count";
const PRINT_COUNT_KEY: &[u8] = b"meta:print_count";
const FOREVER_SEQUENCE_KEY: &[u8] = b"meta:forever_sequence";
const FOREVER_TIP_KEY: &[u8] = b"meta:forever_tip_hash";
const MMR_SCHEMA_KEY: &[u8] = b"meta:mmr_schema_version";
const MMR_LEAF_COUNT_KEY: &[u8] = b"meta:mmr_leaf_count";
const MMR_ROOT_KEY: &[u8] = b"meta:mmr_root_hash";

const NEUROCOGNICA_LOGO_PNG: &[u8] =
    include_bytes!("../../../assets/brand/neurocognica_logo_transparent.png");
const PRINT_CSS: &str = r#"
@page {
  size: Letter;
  margin: 0.55in;
}
* {
  box-sizing: border-box;
}
html,
body {
  margin: 0;
  padding: 0;
  background: #ffffff;
  color: #111111;
  font-family: "Segoe UI", Arial, sans-serif;
  font-size: 10.5pt;
  line-height: 1.48;
}
.sheet {
  min-height: 9.9in;
  border: 2px solid #111111;
  padding: 0.28in;
}
.masthead {
  display: grid;
  grid-template-columns: 1.18in 1fr;
  gap: 0.18in;
  align-items: center;
  border-bottom: 3px solid #111111;
  padding-bottom: 0.16in;
  margin-bottom: 0.16in;
}
.brand-logo {
  width: 1.02in;
  height: 1.02in;
  object-fit: contain;
}
.brand-kicker {
  color: #9a6a1c;
  font-weight: 800;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.brand-title {
  font-size: 30pt;
  font-weight: 850;
  letter-spacing: 0;
  margin-top: 0.03in;
}
.brand-subtitle {
  color: #1b657a;
  font-weight: 800;
  margin-top: 0.04in;
}
.title-block {
  display: grid;
  grid-template-columns: 1.9fr 1fr;
  border: 2px solid #111111;
  margin-bottom: 0.22in;
}
.title-left,
.title-right {
  padding: 0.12in;
}
.title-left {
  border-right: 2px solid #111111;
}
.title-row {
  display: grid;
  grid-template-columns: 0.92in 1fr;
  border-bottom: 1px solid #111111;
  min-height: 0.28in;
}
.title-row:last-child {
  border-bottom: 0;
}
.label {
  font-weight: 850;
  text-transform: uppercase;
}
.value {
  overflow-wrap: anywhere;
}
.document-body {
  min-height: 6.25in;
}
.document-body h1,
.document-body h2,
.document-body h3 {
  break-after: avoid;
  color: #111111;
  letter-spacing: 0;
  margin: 0.16in 0 0.07in;
}
.document-body h1 {
  font-size: 18pt;
}
.document-body h2 {
  font-size: 14pt;
}
.document-body h3 {
  font-size: 12pt;
}
.document-body p {
  margin: 0 0 0.1in;
}
.document-body ul {
  margin: 0 0 0.1in 0.24in;
  padding: 0;
}
.document-body li {
  margin-bottom: 0.04in;
}
.document-body pre {
  border: 1px solid #777777;
  padding: 0.1in;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  font-family: "Cascadia Mono", Consolas, monospace;
  font-size: 8.8pt;
}
.hash-grid {
  display: grid;
  grid-template-columns: 0.98in 1fr;
  gap: 0.02in 0.08in;
  border-top: 1px solid #111111;
  padding-top: 0.1in;
  margin-top: 0.16in;
  font-family: "Cascadia Mono", Consolas, monospace;
  font-size: 7.8pt;
  overflow-wrap: anywhere;
}
.footer {
  border-top: 2px solid #111111;
  margin-top: 0.18in;
  padding-top: 0.08in;
  font-size: 8.2pt;
  font-weight: 700;
}
@media print {
  body {
    print-color-adjust: exact;
    -webkit-print-color-adjust: exact;
  }
  .sheet {
    break-after: page;
  }
}
"#;

#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("unsupported document source extension for {0}")]
    UnsupportedSource(PathBuf),
    #[error("document source must be UTF-8 text: {0}")]
    NonUtf8Source(PathBuf),
    #[error("document source is empty after normalization: {0}")]
    EmptySource(PathBuf),
    #[error("invalid document frame metadata: {0}")]
    InvalidMetadata(String),
    #[error("invalid framer config: {0}")]
    InvalidConfig(String),
    #[error("document store already has a different record for frame {0}")]
    StoreCollision(String),
    #[error("document store corruption: {0}")]
    StoreCorruption(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RocksDb(#[from] rocksdb::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub type DocumentResult<T> = Result<T, DocumentError>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    Markdown,
    PlainText,
    Json,
    JsonLines,
    Csv,
    Tsv,
    Toml,
    Yaml,
}

impl DocumentKind {
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_string_lossy().to_ascii_lowercase();
        match ext.as_str() {
            "md" | "markdown" => Some(Self::Markdown),
            "txt" | "text" => Some(Self::PlainText),
            "json" => Some(Self::Json),
            "jsonl" => Some(Self::JsonLines),
            "csv" => Some(Self::Csv),
            "tsv" => Some(Self::Tsv),
            "toml" => Some(Self::Toml),
            "yaml" | "yml" => Some(Self::Yaml),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NeurocognicaFrameMetadata {
    pub organization: String,
    pub project: String,
    pub title: String,
    pub serialized_id: String,
    pub engineer: String,
    pub date: String,
    pub revision: String,
    pub year: String,
    pub rights: String,
    pub classification: String,
}

impl NeurocognicaFrameMetadata {
    pub fn new(
        project: impl Into<String>,
        title: impl Into<String>,
        serialized_id: impl Into<String>,
        engineer: impl Into<String>,
        date: impl Into<String>,
        revision: impl Into<String>,
        rights: impl Into<String>,
    ) -> Self {
        let year = Utc::now().year().to_string();
        Self {
            organization: "NEUROCOGNICA".to_owned(),
            project: project.into(),
            title: title.into(),
            serialized_id: serialized_id.into(),
            engineer: engineer.into(),
            date: date.into(),
            revision: revision.into(),
            year,
            rights: rights.into(),
            classification: "INTERNAL".to_owned(),
        }
    }

    pub fn validate(&self) -> DocumentResult<()> {
        let required = [
            ("organization", &self.organization),
            ("project", &self.project),
            ("title", &self.title),
            ("serialized_id", &self.serialized_id),
            ("engineer", &self.engineer),
            ("date", &self.date),
            ("revision", &self.revision),
            ("year", &self.year),
            ("rights", &self.rights),
            ("classification", &self.classification),
        ];
        for (field, value) in required {
            if value.trim().is_empty() {
                return Err(DocumentError::InvalidMetadata(format!(
                    "{field} must not be empty"
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramerConfig {
    pub max_chunk_chars: usize,
    pub overlap_chars: usize,
}

impl Default for FramerConfig {
    fn default() -> Self {
        Self {
            max_chunk_chars: DEFAULT_MAX_CHUNK_CHARS,
            overlap_chars: DEFAULT_CHUNK_OVERLAP_CHARS,
        }
    }
}

impl FramerConfig {
    pub fn validate(self) -> DocumentResult<()> {
        if self.max_chunk_chars < 256 {
            return Err(DocumentError::InvalidConfig(
                "max_chunk_chars must be at least 256".to_owned(),
            ));
        }
        if self.overlap_chars >= self.max_chunk_chars {
            return Err(DocumentError::InvalidConfig(
                "overlap_chars must be smaller than max_chunk_chars".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FramerProvenance {
    pub framer: String,
    pub framer_version: String,
    pub inherited_from: String,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentFrameRecord {
    pub schema_version: String,
    pub frame_id: String,
    pub source_path: PathBuf,
    pub source_name: String,
    pub source_kind: DocumentKind,
    pub source_bytes: u64,
    pub source_hash_blake3: String,
    pub text_hash_blake3: String,
    pub metadata_hash_blake3: String,
    pub normalized_text_bytes: u64,
    pub chunk_count: usize,
    pub created_at_utc: chrono::DateTime<Utc>,
    pub metadata: NeurocognicaFrameMetadata,
    pub provenance: FramerProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentChunkRecord {
    pub schema_version: String,
    pub frame_id: String,
    pub chunk_id: String,
    pub chunk_index: usize,
    pub char_start: usize,
    pub char_end: usize,
    pub byte_start: usize,
    pub byte_end: usize,
    pub text_hash_blake3: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentPrintRecord {
    pub schema_version: String,
    pub frame_id: String,
    pub print_id: String,
    pub format: String,
    pub page_profile: String,
    pub logo_asset_name: String,
    pub logo_hash_blake3: String,
    pub html_hash_blake3: String,
    pub generated_at_utc: chrono::DateTime<Utc>,
    pub html: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedDocument {
    pub frame: DocumentFrameRecord,
    pub chunks: Vec<DocumentChunkRecord>,
    pub print: DocumentPrintRecord,
    pub normalized_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentForeverRecord {
    pub schema_version: String,
    pub sequence: u64,
    pub event_id: String,
    pub event_kind: String,
    pub aggregate_id: String,
    pub frame_id: String,
    pub print_id: String,
    pub chunk_count: usize,
    pub payload_hash_blake3: String,
    pub previous_record_hash_blake3: String,
    pub record_hash_blake3: String,
    #[serde(default)]
    pub mmr_schema_version: String,
    #[serde(default)]
    pub mmr_leaf_index: u64,
    #[serde(default)]
    pub mmr_leaf_hash_blake3: String,
    #[serde(default)]
    pub mmr_root_hash_blake3: String,
    #[serde(default)]
    pub mmr_leaf_count: u64,
    pub recorded_at_utc: chrono::DateTime<Utc>,
    pub storage_engine: String,
}

#[derive(Debug)]
pub struct DocumentStore {
    root: PathBuf,
    db_path: PathBuf,
    db: DB,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentStoreSummary {
    pub root: PathBuf,
    pub db_path: PathBuf,
    pub store_engine: String,
    pub frame_count: usize,
    pub chunk_count: usize,
    pub print_count: usize,
    pub forever_event_count: u64,
    pub forever_tip_hash: String,
    pub mmr_leaf_count: u64,
    pub mmr_root_hash: String,
    pub mmr_synced: bool,
    pub legacy_jsonl_rows: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestOutcome {
    Stored {
        frame_id: String,
        print_id: String,
        chunks_written: usize,
        forever_sequence: u64,
        forever_record_hash: String,
    },
    AlreadyExists {
        frame_id: String,
        chunks_present: usize,
        print_present: bool,
        forever_present: bool,
    },
}

impl DocumentStore {
    pub fn open(root: impl Into<PathBuf>) -> DocumentResult<Self> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        let db_path = rocksdb_path(&root);
        let mut options = rocksdb_options();
        options.create_if_missing(true);
        let db = DB::open(&options, &db_path)?;
        let store = Self { root, db_path, db };
        store.ensure_schema()?;
        Ok(store)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn summary(&self) -> DocumentResult<DocumentStoreSummary> {
        let forever_event_count = self.read_counter(FOREVER_SEQUENCE_KEY)?;
        let mmr_leaf_count = self.read_counter(MMR_LEAF_COUNT_KEY)?;
        let mmr_root_hash = self
            .read_string(MMR_ROOT_KEY)?
            .unwrap_or_else(mmr_genesis_hash);
        Ok(DocumentStoreSummary {
            root: self.root.clone(),
            db_path: self.db_path.clone(),
            store_engine: DOCUMENT_STORE_ENGINE.to_owned(),
            frame_count: self.read_counter(FRAME_COUNT_KEY)? as usize,
            chunk_count: self.read_counter(CHUNK_COUNT_KEY)? as usize,
            print_count: self.read_counter(PRINT_COUNT_KEY)? as usize,
            forever_event_count,
            forever_tip_hash: self
                .read_string(FOREVER_TIP_KEY)?
                .unwrap_or_else(genesis_hash),
            mmr_leaf_count,
            mmr_root_hash,
            mmr_synced: mmr_leaf_count == forever_event_count,
            legacy_jsonl_rows: legacy_jsonl_rows(&self.root)?,
        })
    }

    pub fn summary_at(root: impl Into<PathBuf>) -> DocumentResult<DocumentStoreSummary> {
        let root = root.into();
        let db_path = rocksdb_path(&root);
        let legacy_jsonl_rows = legacy_jsonl_rows(&root)?;
        if !db_path.exists() {
            return Ok(DocumentStoreSummary {
                root,
                db_path,
                store_engine: DOCUMENT_STORE_ENGINE.to_owned(),
                frame_count: 0,
                chunk_count: 0,
                print_count: 0,
                forever_event_count: 0,
                forever_tip_hash: genesis_hash(),
                mmr_leaf_count: 0,
                mmr_root_hash: mmr_genesis_hash(),
                mmr_synced: true,
                legacy_jsonl_rows,
            });
        }

        let options = rocksdb_options();
        let db = DB::open_for_read_only(&options, &db_path, false)?;
        let frame_count = read_counter_from_db(&db, FRAME_COUNT_KEY)? as usize;
        let chunk_count = read_counter_from_db(&db, CHUNK_COUNT_KEY)? as usize;
        let print_count = read_counter_from_db(&db, PRINT_COUNT_KEY)? as usize;
        let forever_event_count = read_counter_from_db(&db, FOREVER_SEQUENCE_KEY)?;
        let forever_tip_hash =
            read_string_from_db(&db, FOREVER_TIP_KEY)?.unwrap_or_else(genesis_hash);
        let mmr_leaf_count = read_counter_from_db(&db, MMR_LEAF_COUNT_KEY)?;
        let mmr_root_hash =
            read_string_from_db(&db, MMR_ROOT_KEY)?.unwrap_or_else(mmr_genesis_hash);

        Ok(DocumentStoreSummary {
            root,
            db_path,
            store_engine: DOCUMENT_STORE_ENGINE.to_owned(),
            frame_count,
            chunk_count,
            print_count,
            forever_event_count,
            forever_tip_hash,
            mmr_leaf_count,
            mmr_root_hash,
            mmr_synced: mmr_leaf_count == forever_event_count,
            legacy_jsonl_rows,
        })
    }

    pub fn ingest_text_path(
        &self,
        source_path: &Path,
        metadata: NeurocognicaFrameMetadata,
        config: FramerConfig,
    ) -> DocumentResult<IngestOutcome> {
        let framed = frame_text_document(source_path, metadata, config)?;
        self.ingest_framed_document(&framed)
    }

    pub fn ingest_framed_document(&self, framed: &FramedDocument) -> DocumentResult<IngestOutcome> {
        if self.contains_frame(&framed.frame.frame_id)? {
            let chunks_present = self.count_chunks_for_frame(&framed.frame.frame_id)?;
            let print_present = self.contains_print(&framed.frame.frame_id)?;
            let forever_present = self.contains_forever_event(&framed.frame.frame_id)?;
            return Ok(IngestOutcome::AlreadyExists {
                frame_id: framed.frame.frame_id.clone(),
                chunks_present,
                print_present,
                forever_present,
            });
        }

        let current_frame_count = self.read_counter(FRAME_COUNT_KEY)?;
        let current_chunk_count = self.read_counter(CHUNK_COUNT_KEY)?;
        let current_print_count = self.read_counter(PRINT_COUNT_KEY)?;
        let current_sequence = self.read_counter(FOREVER_SEQUENCE_KEY)?;
        let previous_record_hash = self
            .read_string(FOREVER_TIP_KEY)?
            .unwrap_or_else(genesis_hash);
        let current_mmr_leaf_count = self.read_counter(MMR_LEAF_COUNT_KEY)?;
        if current_mmr_leaf_count != current_sequence {
            return Err(DocumentError::StoreCorruption(format!(
                "document MMR leaf count {current_mmr_leaf_count} does not match forever sequence {current_sequence}"
            )));
        }

        let frame_json = serde_json::to_vec(&framed.frame)?;
        let print_json = serde_json::to_vec(&framed.print)?;
        let chunk_json = framed
            .chunks
            .iter()
            .map(serde_json::to_vec)
            .collect::<Result<Vec<_>, _>>()?;
        let payload =
            DocumentForeverPayload::from_parts(framed, &frame_json, &chunk_json, &print_json);
        let payload_json = serde_json::to_vec(&payload)?;
        let payload_hash_blake3 = blake3_with_prefix(&payload_json);
        let forever_sequence = current_sequence.checked_add(1).ok_or_else(|| {
            DocumentError::StoreCorruption("forever sequence overflow".to_owned())
        })?;
        let forever_record = build_forever_record(
            forever_sequence,
            &framed.frame.frame_id,
            &framed.print.print_id,
            framed.chunks.len(),
            &payload_hash_blake3,
            &previous_record_hash,
        )?;
        let mut forever_record = forever_record;
        let mmr_leaf_hash =
            compute_mmr_leaf_hash(forever_sequence, &forever_record.record_hash_blake3)?;
        let mmr_append = self.plan_mmr_append(current_mmr_leaf_count, &mmr_leaf_hash)?;
        forever_record.mmr_schema_version = DOCUMENT_MMR_SCHEMA_VERSION.to_owned();
        forever_record.mmr_leaf_index = mmr_append.leaf_index;
        forever_record.mmr_leaf_hash_blake3 = mmr_append.leaf_hash.clone();
        forever_record.mmr_root_hash_blake3 = mmr_append.root_hash.clone();
        forever_record.mmr_leaf_count = mmr_append.leaf_count;
        let forever_json = serde_json::to_vec(&forever_record)?;

        let mut batch = WriteBatch::default();
        batch.put(frame_key(&framed.frame.frame_id), frame_json);
        batch.put(
            source_text_key(&framed.frame.frame_id),
            framed.normalized_text.as_bytes(),
        );
        batch.put(
            chunk_count_key(&framed.frame.frame_id),
            framed.chunks.len().to_string().into_bytes(),
        );
        for (chunk, chunk_bytes) in framed.chunks.iter().zip(chunk_json) {
            batch.put(
                chunk_key(&framed.frame.frame_id, chunk.chunk_index),
                chunk_bytes,
            );
            batch.put(
                chunk_id_key(&chunk.chunk_id),
                framed.frame.frame_id.as_bytes(),
            );
        }
        batch.put(print_key(&framed.print.print_id), print_json);
        batch.put(
            print_by_frame_key(&framed.frame.frame_id),
            framed.print.print_id.as_bytes(),
        );
        batch.put(forever_key(forever_sequence), forever_json);
        batch.put(
            forever_by_frame_key(&framed.frame.frame_id),
            forever_sequence.to_string().into_bytes(),
        );
        batch.put(
            FRAME_COUNT_KEY,
            current_frame_count
                .checked_add(1)
                .ok_or_else(|| DocumentError::StoreCorruption("frame count overflow".to_owned()))?
                .to_string()
                .into_bytes(),
        );
        batch.put(
            CHUNK_COUNT_KEY,
            current_chunk_count
                .checked_add(framed.chunks.len() as u64)
                .ok_or_else(|| DocumentError::StoreCorruption("chunk count overflow".to_owned()))?
                .to_string()
                .into_bytes(),
        );
        batch.put(
            PRINT_COUNT_KEY,
            current_print_count
                .checked_add(1)
                .ok_or_else(|| DocumentError::StoreCorruption("print count overflow".to_owned()))?
                .to_string()
                .into_bytes(),
        );
        batch.put(
            FOREVER_SEQUENCE_KEY,
            forever_sequence.to_string().into_bytes(),
        );
        batch.put(
            FOREVER_TIP_KEY,
            forever_record.record_hash_blake3.as_bytes(),
        );
        batch.put(MMR_SCHEMA_KEY, DOCUMENT_MMR_SCHEMA_VERSION.as_bytes());
        batch.put(
            mmr_leaf_key(mmr_append.leaf_index),
            mmr_append.leaf_hash.as_bytes(),
        );
        batch.put(
            mmr_event_key(forever_sequence),
            mmr_append.leaf_index.to_string().into_bytes(),
        );
        for height in &mmr_append.peak_deletes {
            batch.delete(mmr_peak_key(*height));
        }
        for peak in &mmr_append.peak_puts {
            batch.put(mmr_peak_key(peak.height), peak.hash.as_bytes());
        }
        batch.put(
            MMR_LEAF_COUNT_KEY,
            mmr_append.leaf_count.to_string().into_bytes(),
        );
        batch.put(MMR_ROOT_KEY, mmr_append.root_hash.as_bytes());

        self.write_synced(batch)?;

        Ok(IngestOutcome::Stored {
            frame_id: framed.frame.frame_id.clone(),
            print_id: framed.print.print_id.clone(),
            chunks_written: framed.chunks.len(),
            forever_sequence,
            forever_record_hash: forever_record.record_hash_blake3,
        })
    }

    pub fn frame_record(&self, frame_id: &str) -> DocumentResult<Option<DocumentFrameRecord>> {
        self.get_json(frame_key(frame_id))
    }

    pub fn print_record_for_frame(
        &self,
        frame_id: &str,
    ) -> DocumentResult<Option<DocumentPrintRecord>> {
        let Some(print_id) = self.read_string(&print_by_frame_key(frame_id))? else {
            return Ok(None);
        };
        self.get_json(print_key(&print_id))
    }

    pub fn forever_record_for_frame(
        &self,
        frame_id: &str,
    ) -> DocumentResult<Option<DocumentForeverRecord>> {
        let Some(sequence) = self.read_string(&forever_by_frame_key(frame_id))? else {
            return Ok(None);
        };
        let sequence = parse_counter(&sequence, "forever_by_frame")?;
        self.get_json(forever_key(sequence))
    }

    pub fn verify_forever_chain(&self) -> DocumentResult<ForeverChainReport> {
        let event_count = self.read_counter(FOREVER_SEQUENCE_KEY)?;
        let mut expected_previous = genesis_hash();

        for sequence in 1..=event_count {
            let record: DocumentForeverRecord =
                self.get_json(forever_key(sequence))?.ok_or_else(|| {
                    DocumentError::StoreCorruption(format!(
                        "missing forever record at sequence {sequence}"
                    ))
                })?;
            if record.sequence != sequence {
                return Err(DocumentError::StoreCorruption(format!(
                    "forever record sequence mismatch at {sequence}: found {}",
                    record.sequence
                )));
            }
            if record.previous_record_hash_blake3 != expected_previous {
                return Err(DocumentError::StoreCorruption(format!(
                    "forever record {sequence} previous hash mismatch"
                )));
            }
            let expected_hash = compute_forever_record_hash(&record)?;
            if record.record_hash_blake3 != expected_hash {
                return Err(DocumentError::StoreCorruption(format!(
                    "forever record {sequence} hash mismatch"
                )));
            }
            expected_previous = record.record_hash_blake3;
        }

        let tip_hash = self
            .read_string(FOREVER_TIP_KEY)?
            .unwrap_or_else(genesis_hash);
        if tip_hash != expected_previous {
            return Err(DocumentError::StoreCorruption(
                "forever tip does not match verified chain".to_owned(),
            ));
        }

        Ok(ForeverChainReport {
            event_count,
            tip_hash,
            verified: true,
        })
    }

    pub fn verify_mmr(&self) -> DocumentResult<DocumentMmrReport> {
        let event_count = self.read_counter(FOREVER_SEQUENCE_KEY)?;
        if event_count > 0 {
            let schema = self.read_string(MMR_SCHEMA_KEY)?.ok_or_else(|| {
                DocumentError::StoreCorruption(
                    "document MMR schema is missing for non-empty store".to_owned(),
                )
            })?;
            if schema != DOCUMENT_MMR_SCHEMA_VERSION {
                return Err(DocumentError::StoreCorruption(format!(
                    "unsupported document MMR schema {schema}"
                )));
            }
        }

        let mut peaks = BTreeMap::new();
        let mut leaf_count = 0_u64;

        for sequence in 1..=event_count {
            let record: DocumentForeverRecord =
                self.get_json(forever_key(sequence))?.ok_or_else(|| {
                    DocumentError::StoreCorruption(format!(
                        "missing forever record at sequence {sequence}"
                    ))
                })?;
            if record.mmr_schema_version != DOCUMENT_MMR_SCHEMA_VERSION {
                return Err(DocumentError::StoreCorruption(format!(
                    "forever record {sequence} is not bound to document MMR schema"
                )));
            }
            if record.sequence != sequence {
                return Err(DocumentError::StoreCorruption(format!(
                    "forever record sequence mismatch at {sequence}: found {}",
                    record.sequence
                )));
            }
            let expected_record_hash = compute_forever_record_hash(&record)?;
            if record.record_hash_blake3 != expected_record_hash {
                return Err(DocumentError::StoreCorruption(format!(
                    "forever record {sequence} hash mismatch"
                )));
            }

            let expected_leaf_index = sequence.checked_sub(1).ok_or_else(|| {
                DocumentError::StoreCorruption("MMR sequence underflow".to_owned())
            })?;
            if record.mmr_leaf_index != expected_leaf_index {
                return Err(DocumentError::StoreCorruption(format!(
                    "MMR leaf index mismatch for sequence {sequence}"
                )));
            }
            let expected_leaf_hash = compute_mmr_leaf_hash(sequence, &record.record_hash_blake3)?;
            if record.mmr_leaf_hash_blake3 != expected_leaf_hash {
                return Err(DocumentError::StoreCorruption(format!(
                    "MMR leaf hash mismatch for sequence {sequence}"
                )));
            }
            let stored_leaf_hash = self
                .read_string(mmr_leaf_key(expected_leaf_index))?
                .ok_or_else(|| {
                    DocumentError::StoreCorruption(format!(
                        "missing MMR leaf at index {expected_leaf_index}"
                    ))
                })?;
            if stored_leaf_hash != expected_leaf_hash {
                return Err(DocumentError::StoreCorruption(format!(
                    "stored MMR leaf mismatch at index {expected_leaf_index}"
                )));
            }
            let stored_leaf_index =
                self.read_string(mmr_event_key(sequence))?.ok_or_else(|| {
                    DocumentError::StoreCorruption(format!(
                        "missing MMR event index for sequence {sequence}"
                    ))
                })?;
            if parse_counter(&stored_leaf_index, "mmr event leaf index")? != expected_leaf_index {
                return Err(DocumentError::StoreCorruption(format!(
                    "MMR event index mismatch for sequence {sequence}"
                )));
            }

            append_mmr_leaf_to_peaks(&mut peaks, &expected_leaf_hash)?;
            leaf_count = leaf_count.checked_add(1).ok_or_else(|| {
                DocumentError::StoreCorruption("MMR leaf count overflow".to_owned())
            })?;
            let expected_root = compute_mmr_root(leaf_count, &peaks)?;
            if record.mmr_leaf_count != leaf_count {
                return Err(DocumentError::StoreCorruption(format!(
                    "MMR leaf count mismatch for sequence {sequence}"
                )));
            }
            if record.mmr_root_hash_blake3 != expected_root {
                return Err(DocumentError::StoreCorruption(format!(
                    "MMR root mismatch for sequence {sequence}"
                )));
            }
        }

        let stored_leaf_count = self.read_counter(MMR_LEAF_COUNT_KEY)?;
        if stored_leaf_count != leaf_count {
            return Err(DocumentError::StoreCorruption(format!(
                "stored MMR leaf count {stored_leaf_count} does not match replayed {leaf_count}"
            )));
        }

        let expected_root = compute_mmr_root(leaf_count, &peaks)?;
        let stored_root = self
            .read_string(MMR_ROOT_KEY)?
            .unwrap_or_else(mmr_genesis_hash);
        if stored_root != expected_root {
            return Err(DocumentError::StoreCorruption(
                "stored MMR root does not match replay".to_owned(),
            ));
        }

        let stored_peaks = self.read_mmr_peaks()?;
        if stored_peaks != peaks {
            return Err(DocumentError::StoreCorruption(
                "stored MMR peaks do not match replay".to_owned(),
            ));
        }

        Ok(DocumentMmrReport {
            leaf_count,
            root_hash: stored_root,
            peak_count: peaks.len(),
            verified: true,
        })
    }

    fn contains_frame(&self, frame_id: &str) -> DocumentResult<bool> {
        Ok(self.db.get(frame_key(frame_id))?.is_some())
    }

    fn count_chunks_for_frame(&self, frame_id: &str) -> DocumentResult<usize> {
        Ok(self.read_counter(&chunk_count_key(frame_id))? as usize)
    }

    fn contains_print(&self, frame_id: &str) -> DocumentResult<bool> {
        Ok(self.db.get(print_by_frame_key(frame_id))?.is_some())
    }

    fn contains_forever_event(&self, frame_id: &str) -> DocumentResult<bool> {
        Ok(self.db.get(forever_by_frame_key(frame_id))?.is_some())
    }

    fn ensure_schema(&self) -> DocumentResult<()> {
        if let Some(schema) = self.read_string(STORE_SCHEMA_KEY)? {
            if schema != DOCUMENT_STORE_SCHEMA_VERSION {
                return Err(DocumentError::StoreCorruption(format!(
                    "unsupported document store schema {schema}"
                )));
            }
            self.ensure_mmr_schema()?;
            return Ok(());
        }

        let mut batch = WriteBatch::default();
        batch.put(STORE_SCHEMA_KEY, DOCUMENT_STORE_SCHEMA_VERSION.as_bytes());
        batch.put(FRAME_COUNT_KEY, b"0");
        batch.put(CHUNK_COUNT_KEY, b"0");
        batch.put(PRINT_COUNT_KEY, b"0");
        batch.put(FOREVER_SEQUENCE_KEY, b"0");
        batch.put(FOREVER_TIP_KEY, genesis_hash().as_bytes());
        batch.put(MMR_SCHEMA_KEY, DOCUMENT_MMR_SCHEMA_VERSION.as_bytes());
        batch.put(MMR_LEAF_COUNT_KEY, b"0");
        batch.put(MMR_ROOT_KEY, mmr_genesis_hash().as_bytes());
        self.write_synced(batch)
    }

    fn ensure_mmr_schema(&self) -> DocumentResult<()> {
        if let Some(schema) = self.read_string(MMR_SCHEMA_KEY)? {
            if schema != DOCUMENT_MMR_SCHEMA_VERSION {
                return Err(DocumentError::StoreCorruption(format!(
                    "unsupported document MMR schema {schema}"
                )));
            }
            return Ok(());
        }

        if self.read_counter(FOREVER_SEQUENCE_KEY)? > 0 {
            return Ok(());
        }

        let mut batch = WriteBatch::default();
        batch.put(MMR_SCHEMA_KEY, DOCUMENT_MMR_SCHEMA_VERSION.as_bytes());
        batch.put(MMR_LEAF_COUNT_KEY, b"0");
        batch.put(MMR_ROOT_KEY, mmr_genesis_hash().as_bytes());
        self.write_synced(batch)
    }

    fn plan_mmr_append(&self, leaf_index: u64, leaf_hash: &str) -> DocumentResult<MmrAppendPlan> {
        let mut peaks = self.read_mmr_peaks()?;
        let mut peak_deletes = Vec::new();
        let mut peak_puts = Vec::new();
        let mut carry_hash = leaf_hash.to_owned();
        let mut height = 0_u32;

        while let Some(left_hash) = peaks.remove(&height) {
            peak_deletes.push(height);
            carry_hash = compute_mmr_parent_hash(height, &left_hash, &carry_hash)?;
            height = height.checked_add(1).ok_or_else(|| {
                DocumentError::StoreCorruption("MMR peak height overflow".to_owned())
            })?;
        }

        peaks.insert(height, carry_hash.clone());
        peak_puts.push(MmrPeak {
            height,
            hash: carry_hash,
        });

        let leaf_count = leaf_index
            .checked_add(1)
            .ok_or_else(|| DocumentError::StoreCorruption("MMR leaf count overflow".to_owned()))?;
        let root_hash = compute_mmr_root(leaf_count, &peaks)?;

        Ok(MmrAppendPlan {
            leaf_index,
            leaf_hash: leaf_hash.to_owned(),
            leaf_count,
            root_hash,
            peak_deletes,
            peak_puts,
        })
    }

    fn read_mmr_peaks(&self) -> DocumentResult<BTreeMap<u32, String>> {
        let leaf_count = self.read_counter(MMR_LEAF_COUNT_KEY)?;
        let max_height = u64::BITS - leaf_count.leading_zeros();
        let mut peaks = BTreeMap::new();
        for height in 0..=max_height {
            if let Some(hash) = self.read_string(mmr_peak_key(height))? {
                peaks.insert(height, hash);
            }
        }
        Ok(peaks)
    }

    fn read_counter<K: AsRef<[u8]>>(&self, key: K) -> DocumentResult<u64> {
        read_counter_from_db(&self.db, key)
    }

    fn read_string<K: AsRef<[u8]>>(&self, key: K) -> DocumentResult<Option<String>> {
        read_string_from_db(&self.db, key)
    }

    fn get_json<T: for<'de> Deserialize<'de>, K: AsRef<[u8]>>(
        &self,
        key: K,
    ) -> DocumentResult<Option<T>> {
        let Some(bytes) = self.db.get(key)? else {
            return Ok(None);
        };
        Ok(Some(serde_json::from_slice(&bytes)?))
    }

    fn write_synced(&self, batch: WriteBatch) -> DocumentResult<()> {
        let mut options = WriteOptions::default();
        options.set_sync(true);
        self.db.write_opt(batch, &options)?;
        self.db.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeverChainReport {
    pub event_count: u64,
    pub tip_hash: String,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentMmrReport {
    pub leaf_count: u64,
    pub root_hash: String,
    pub peak_count: usize,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MmrPeak {
    height: u32,
    hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MmrAppendPlan {
    leaf_index: u64,
    leaf_hash: String,
    leaf_count: u64,
    root_hash: String,
    peak_deletes: Vec<u32>,
    peak_puts: Vec<MmrPeak>,
}

#[derive(Debug, Serialize)]
struct DocumentForeverPayload {
    frame_id: String,
    print_id: String,
    source_hash_blake3: String,
    text_hash_blake3: String,
    metadata_hash_blake3: String,
    print_html_hash_blake3: String,
    frame_json_hash_blake3: String,
    print_json_hash_blake3: String,
    chunk_record_hashes_blake3: Vec<String>,
    normalized_text_hash_blake3: String,
    chunk_count: usize,
}

impl DocumentForeverPayload {
    fn from_parts(
        framed: &FramedDocument,
        frame_json: &[u8],
        chunk_json: &[Vec<u8>],
        print_json: &[u8],
    ) -> Self {
        Self {
            frame_id: framed.frame.frame_id.clone(),
            print_id: framed.print.print_id.clone(),
            source_hash_blake3: framed.frame.source_hash_blake3.clone(),
            text_hash_blake3: framed.frame.text_hash_blake3.clone(),
            metadata_hash_blake3: framed.frame.metadata_hash_blake3.clone(),
            print_html_hash_blake3: framed.print.html_hash_blake3.clone(),
            frame_json_hash_blake3: blake3_with_prefix(frame_json),
            print_json_hash_blake3: blake3_with_prefix(print_json),
            chunk_record_hashes_blake3: chunk_json
                .iter()
                .map(|chunk| blake3_with_prefix(chunk))
                .collect(),
            normalized_text_hash_blake3: blake3_with_prefix(framed.normalized_text.as_bytes()),
            chunk_count: framed.chunks.len(),
        }
    }
}

#[derive(Debug, Serialize)]
struct DocumentForeverRecordSeal<'a> {
    schema_version: &'a str,
    sequence: u64,
    event_id: &'a str,
    event_kind: &'a str,
    aggregate_id: &'a str,
    frame_id: &'a str,
    print_id: &'a str,
    chunk_count: usize,
    payload_hash_blake3: &'a str,
    previous_record_hash_blake3: &'a str,
    recorded_at_utc: chrono::DateTime<Utc>,
    storage_engine: &'a str,
}

#[derive(Debug, Serialize)]
struct DocumentMmrLeafSeal<'a> {
    schema_version: &'a str,
    leaf_kind: &'a str,
    sequence: u64,
    record_hash_blake3: &'a str,
}

#[derive(Debug, Serialize)]
struct DocumentMmrParentSeal<'a> {
    schema_version: &'a str,
    child_height: u32,
    left_hash_blake3: &'a str,
    right_hash_blake3: &'a str,
}

#[derive(Debug, Serialize)]
struct DocumentMmrRootSeal<'a> {
    schema_version: &'a str,
    leaf_count: u64,
    peaks: Vec<DocumentMmrPeakSeal<'a>>,
}

#[derive(Debug, Serialize)]
struct DocumentMmrPeakSeal<'a> {
    height: u32,
    hash_blake3: &'a str,
}

fn build_forever_record(
    sequence: u64,
    frame_id: &str,
    print_id: &str,
    chunk_count: usize,
    payload_hash_blake3: &str,
    previous_record_hash_blake3: &str,
) -> DocumentResult<DocumentForeverRecord> {
    let recorded_at_utc = Utc::now();
    let event_kind = "document.ingested";
    let aggregate_id = frame_id;
    let event_id_seed =
        format!("{sequence}:{event_kind}:{frame_id}:{print_id}:{payload_hash_blake3}");
    let event_id_hash = blake3_hex(event_id_seed.as_bytes());
    let event_id = format!("ncfv-{}", &event_id_hash[..24]);
    let seal = DocumentForeverRecordSeal {
        schema_version: DOCUMENT_FOREVER_SCHEMA_VERSION,
        sequence,
        event_id: &event_id,
        event_kind,
        aggregate_id,
        frame_id,
        print_id,
        chunk_count,
        payload_hash_blake3,
        previous_record_hash_blake3,
        recorded_at_utc,
        storage_engine: DOCUMENT_STORE_ENGINE,
    };
    let record_hash_blake3 = blake3_with_prefix(&serde_json::to_vec(&seal)?);

    Ok(DocumentForeverRecord {
        schema_version: DOCUMENT_FOREVER_SCHEMA_VERSION.to_owned(),
        sequence,
        event_id,
        event_kind: event_kind.to_owned(),
        aggregate_id: aggregate_id.to_owned(),
        frame_id: frame_id.to_owned(),
        print_id: print_id.to_owned(),
        chunk_count,
        payload_hash_blake3: payload_hash_blake3.to_owned(),
        previous_record_hash_blake3: previous_record_hash_blake3.to_owned(),
        record_hash_blake3,
        mmr_schema_version: String::new(),
        mmr_leaf_index: 0,
        mmr_leaf_hash_blake3: String::new(),
        mmr_root_hash_blake3: String::new(),
        mmr_leaf_count: 0,
        recorded_at_utc,
        storage_engine: DOCUMENT_STORE_ENGINE.to_owned(),
    })
}

fn compute_forever_record_hash(record: &DocumentForeverRecord) -> DocumentResult<String> {
    let seal = DocumentForeverRecordSeal {
        schema_version: &record.schema_version,
        sequence: record.sequence,
        event_id: &record.event_id,
        event_kind: &record.event_kind,
        aggregate_id: &record.aggregate_id,
        frame_id: &record.frame_id,
        print_id: &record.print_id,
        chunk_count: record.chunk_count,
        payload_hash_blake3: &record.payload_hash_blake3,
        previous_record_hash_blake3: &record.previous_record_hash_blake3,
        recorded_at_utc: record.recorded_at_utc,
        storage_engine: &record.storage_engine,
    };
    Ok(blake3_with_prefix(&serde_json::to_vec(&seal)?))
}

fn compute_mmr_leaf_hash(sequence: u64, record_hash_blake3: &str) -> DocumentResult<String> {
    let seal = DocumentMmrLeafSeal {
        schema_version: DOCUMENT_MMR_SCHEMA_VERSION,
        leaf_kind: "document_forever_record",
        sequence,
        record_hash_blake3,
    };
    Ok(blake3_with_prefix(&serde_json::to_vec(&seal)?))
}

fn compute_mmr_parent_hash(
    child_height: u32,
    left_hash_blake3: &str,
    right_hash_blake3: &str,
) -> DocumentResult<String> {
    let seal = DocumentMmrParentSeal {
        schema_version: DOCUMENT_MMR_SCHEMA_VERSION,
        child_height,
        left_hash_blake3,
        right_hash_blake3,
    };
    Ok(blake3_with_prefix(&serde_json::to_vec(&seal)?))
}

fn append_mmr_leaf_to_peaks(
    peaks: &mut BTreeMap<u32, String>,
    leaf_hash: &str,
) -> DocumentResult<()> {
    let mut carry_hash = leaf_hash.to_owned();
    let mut height = 0_u32;
    while let Some(left_hash) = peaks.remove(&height) {
        carry_hash = compute_mmr_parent_hash(height, &left_hash, &carry_hash)?;
        height = height
            .checked_add(1)
            .ok_or_else(|| DocumentError::StoreCorruption("MMR peak height overflow".to_owned()))?;
    }
    peaks.insert(height, carry_hash);
    Ok(())
}

fn compute_mmr_root(leaf_count: u64, peaks: &BTreeMap<u32, String>) -> DocumentResult<String> {
    if leaf_count == 0 {
        return Ok(mmr_genesis_hash());
    }
    let peak_seals = peaks
        .iter()
        .rev()
        .map(|(height, hash)| DocumentMmrPeakSeal {
            height: *height,
            hash_blake3: hash.as_str(),
        })
        .collect();
    let seal = DocumentMmrRootSeal {
        schema_version: DOCUMENT_MMR_SCHEMA_VERSION,
        leaf_count,
        peaks: peak_seals,
    };
    Ok(blake3_with_prefix(&serde_json::to_vec(&seal)?))
}

fn rocksdb_options() -> Options {
    let mut options = Options::default();
    options.create_if_missing(false);
    options
}

fn rocksdb_path(root: &Path) -> PathBuf {
    root.join(DOCUMENT_ROCKSDB_DIR_NAME)
}

fn read_counter_from_db<K: AsRef<[u8]>>(db: &DB, key: K) -> DocumentResult<u64> {
    let Some(value) = read_string_from_db(db, key)? else {
        return Ok(0);
    };
    parse_counter(&value, "counter")
}

fn read_string_from_db<K: AsRef<[u8]>>(db: &DB, key: K) -> DocumentResult<Option<String>> {
    let Some(value) = db.get(key)? else {
        return Ok(None);
    };
    String::from_utf8(value)
        .map(Some)
        .map_err(|error| DocumentError::StoreCorruption(format!("invalid utf-8 value: {error}")))
}

fn parse_counter(value: &str, label: &str) -> DocumentResult<u64> {
    value.parse::<u64>().map_err(|error| {
        DocumentError::StoreCorruption(format!("invalid {label} counter {value}: {error}"))
    })
}

fn frame_key(frame_id: &str) -> Vec<u8> {
    format!("frame:{frame_id}").into_bytes()
}

fn source_text_key(frame_id: &str) -> Vec<u8> {
    format!("source_text:{frame_id}").into_bytes()
}

fn chunk_count_key(frame_id: &str) -> Vec<u8> {
    format!("chunk_count:{frame_id}").into_bytes()
}

fn chunk_key(frame_id: &str, chunk_index: usize) -> Vec<u8> {
    format!("chunk:{frame_id}:{chunk_index:08}").into_bytes()
}

fn chunk_id_key(chunk_id: &str) -> Vec<u8> {
    format!("chunk_id:{chunk_id}").into_bytes()
}

fn print_key(print_id: &str) -> Vec<u8> {
    format!("print:{print_id}").into_bytes()
}

fn print_by_frame_key(frame_id: &str) -> Vec<u8> {
    format!("print_by_frame:{frame_id}").into_bytes()
}

fn forever_key(sequence: u64) -> Vec<u8> {
    format!("forever:{sequence:020}").into_bytes()
}

fn forever_by_frame_key(frame_id: &str) -> Vec<u8> {
    format!("forever_by_frame:{frame_id}").into_bytes()
}

fn mmr_leaf_key(leaf_index: u64) -> Vec<u8> {
    format!("mmr:leaf:{leaf_index:020}").into_bytes()
}

fn mmr_event_key(sequence: u64) -> Vec<u8> {
    format!("mmr:event:{sequence:020}").into_bytes()
}

fn mmr_peak_key(height: u32) -> Vec<u8> {
    format!("mmr:peak:{height:03}").into_bytes()
}

fn legacy_jsonl_rows(root: &Path) -> DocumentResult<usize> {
    Ok(count_jsonl_rows_if_exists(&frames_path(root))?
        + count_jsonl_rows_if_exists(&chunks_path(root))?
        + count_jsonl_rows_if_exists(&prints_path(root))?)
}

fn count_jsonl_rows_if_exists(path: &Path) -> DocumentResult<usize> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(error) => Err(error.into()),
    }
}

pub fn default_document_dir(data_dir: &Path) -> PathBuf {
    data_dir.join("documents")
}

pub fn frame_text_document(
    source_path: &Path,
    metadata: NeurocognicaFrameMetadata,
    config: FramerConfig,
) -> DocumentResult<FramedDocument> {
    metadata.validate()?;
    config.validate()?;

    let source_kind = DocumentKind::from_path(source_path)
        .ok_or_else(|| DocumentError::UnsupportedSource(source_path.to_path_buf()))?;
    let raw = fs::read(source_path)?;
    let source_hash_blake3 = blake3_hex(&raw);
    let source_bytes = raw.len() as u64;
    let text =
        String::from_utf8(raw).map_err(|_| DocumentError::NonUtf8Source(source_path.into()))?;
    let normalized_text = normalize_text(&text);
    if normalized_text.trim().is_empty() {
        return Err(DocumentError::EmptySource(source_path.to_path_buf()));
    }

    let text_hash_blake3 = blake3_hex(normalized_text.as_bytes());
    let metadata_hash_blake3 = blake3_hex(&serde_json::to_vec(&metadata)?);
    let frame_seed = format!("{source_hash_blake3}:{text_hash_blake3}:{metadata_hash_blake3}");
    let frame_hash = blake3_hex(frame_seed.as_bytes());
    let frame_id = format!("ncdf-{}", &frame_hash[..24]);
    let spans = chunk_spans(&normalized_text, config);
    let source_name = source_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "document".to_owned());
    let created_at_utc = Utc::now();
    let print = build_print_record(
        &frame_id,
        source_path,
        &source_name,
        source_kind,
        source_bytes,
        &source_hash_blake3,
        &text_hash_blake3,
        &metadata_hash_blake3,
        normalized_text.as_bytes().len() as u64,
        spans.len(),
        created_at_utc,
        &metadata,
        &normalized_text,
    );

    let frame = DocumentFrameRecord {
        schema_version: FRAME_SCHEMA_VERSION.to_owned(),
        frame_id: frame_id.clone(),
        source_path: source_path.to_path_buf(),
        source_name,
        source_kind,
        source_bytes,
        source_hash_blake3,
        text_hash_blake3,
        metadata_hash_blake3,
        normalized_text_bytes: normalized_text.as_bytes().len() as u64,
        chunk_count: spans.len(),
        created_at_utc,
        metadata,
        provenance: FramerProvenance {
            framer: "aura_documents".to_owned(),
            framer_version: env!("CARGO_PKG_VERSION").to_owned(),
            inherited_from: r"C:\AURA-Lab\Doc_Framer\nc-framer.py".to_owned(),
            rule: "Every integrated AURA document must be framed and print-ready before storage."
                .to_owned(),
        },
    };

    let chunks = spans
        .into_iter()
        .enumerate()
        .map(|(chunk_index, span)| {
            let text = normalized_text[span.byte_start..span.byte_end].to_owned();
            let text_hash_blake3 = blake3_hex(text.as_bytes());
            let chunk_id = format!("{frame_id}-{:04}-{}", chunk_index, &text_hash_blake3[..12]);
            DocumentChunkRecord {
                schema_version: CHUNK_SCHEMA_VERSION.to_owned(),
                frame_id: frame_id.clone(),
                chunk_id,
                chunk_index,
                char_start: span.char_start,
                char_end: span.char_end,
                byte_start: span.byte_start,
                byte_end: span.byte_end,
                text_hash_blake3,
                text,
            }
        })
        .collect::<Vec<_>>();

    Ok(FramedDocument {
        frame,
        chunks,
        print,
        normalized_text,
    })
}

struct PrintRenderContext<'a> {
    frame_id: &'a str,
    source_path: &'a Path,
    source_name: &'a str,
    source_kind: DocumentKind,
    source_bytes: u64,
    source_hash_blake3: &'a str,
    text_hash_blake3: &'a str,
    metadata_hash_blake3: &'a str,
    normalized_text_bytes: u64,
    chunk_count: usize,
    generated_at_utc: chrono::DateTime<Utc>,
    metadata: &'a NeurocognicaFrameMetadata,
    normalized_text: &'a str,
    logo_data_uri: &'a str,
    logo_hash_blake3: &'a str,
}

fn build_print_record(
    frame_id: &str,
    source_path: &Path,
    source_name: &str,
    source_kind: DocumentKind,
    source_bytes: u64,
    source_hash_blake3: &str,
    text_hash_blake3: &str,
    metadata_hash_blake3: &str,
    normalized_text_bytes: u64,
    chunk_count: usize,
    generated_at_utc: chrono::DateTime<Utc>,
    metadata: &NeurocognicaFrameMetadata,
    normalized_text: &str,
) -> DocumentPrintRecord {
    let logo_hash_blake3 = blake3_hex(NEUROCOGNICA_LOGO_PNG);
    let logo_data_uri = format!(
        "data:image/png;base64,{}",
        BASE64.encode(NEUROCOGNICA_LOGO_PNG)
    );
    let context = PrintRenderContext {
        frame_id,
        source_path,
        source_name,
        source_kind,
        source_bytes,
        source_hash_blake3,
        text_hash_blake3,
        metadata_hash_blake3,
        normalized_text_bytes,
        chunk_count,
        generated_at_utc,
        metadata,
        normalized_text,
        logo_data_uri: &logo_data_uri,
        logo_hash_blake3: &logo_hash_blake3,
    };
    let html = render_print_html(&context);
    let html_hash_blake3 = blake3_hex(html.as_bytes());
    let print_seed = format!(
        "{frame_id}:{PRINT_SCHEMA_VERSION}:{PRINT_PAGE_PROFILE}:{PRINT_LOGO_ASSET_NAME}:{logo_hash_blake3}"
    );
    let print_hash = blake3_hex(print_seed.as_bytes());
    let print_id = format!("ncdp-{}", &print_hash[..24]);

    DocumentPrintRecord {
        schema_version: PRINT_SCHEMA_VERSION.to_owned(),
        frame_id: frame_id.to_owned(),
        print_id,
        format: PRINT_FORMAT_HTML.to_owned(),
        page_profile: PRINT_PAGE_PROFILE.to_owned(),
        logo_asset_name: PRINT_LOGO_ASSET_NAME.to_owned(),
        logo_hash_blake3,
        html_hash_blake3,
        generated_at_utc,
        html,
    }
}

fn render_print_html(context: &PrintRenderContext<'_>) -> String {
    let metadata = context.metadata;
    let body = render_print_body(context.normalized_text);
    let source_path = context.source_path.display().to_string();
    let generated_at = context.generated_at_utc.to_rfc3339();
    let kind = document_kind_label(context.source_kind);

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title} - AURA Print Frame</title>
<style>{style}</style>
</head>
<body>
<main class="sheet">
  <header class="masthead">
    <img class="brand-logo" src="{logo_data_uri}" alt="NeuroCognica logo">
    <div>
      <div class="brand-kicker">NEUROCOGNICA</div>
      <div class="brand-title">AURA</div>
      <div class="brand-subtitle">Archetypes - Utilizing - Reflective - Architecture</div>
    </div>
  </header>

  <section class="title-block" aria-label="NeuroCognica title block">
    <div class="title-left">
      <div class="title-row"><span class="label">PROJECT:</span><span class="value">{project}</span></div>
      <div class="title-row"><span class="label">TITLE:</span><span class="value">{title}</span></div>
      <div class="title-row"><span class="label">SOURCE:</span><span class="value">{source_name}</span></div>
      <div class="title-row"><span class="label">PATH:</span><span class="value">{source_path}</span></div>
    </div>
    <div class="title-right">
      <div class="title-row"><span class="label">DWG NO:</span><span class="value">{serialized_id}</span></div>
      <div class="title-row"><span class="label">ENGR:</span><span class="value">{engineer}</span></div>
      <div class="title-row"><span class="label">DATE:</span><span class="value">{date}</span></div>
      <div class="title-row"><span class="label">REV:</span><span class="value">{revision}</span></div>
      <div class="title-row"><span class="label">CLASS:</span><span class="value">{classification}</span></div>
    </div>
  </section>

  <section class="document-body">
    {body}
  </section>

  <section class="hash-grid" aria-label="AURA document hashes">
    <div>FRAME</div><div>{frame_id}</div>
    <div>FORMAT</div><div>{format} / {page_profile} / {kind}</div>
    <div>SOURCE</div><div>{source_bytes} bytes / {source_hash}</div>
    <div>TEXT</div><div>{normalized_text_bytes} bytes / {text_hash}</div>
    <div>META</div><div>{metadata_hash}</div>
    <div>LOGO</div><div>{logo_asset} / {logo_hash}</div>
    <div>CHUNKS</div><div>{chunk_count}</div>
    <div>SEALED</div><div>{generated_at}</div>
  </section>

  <footer class="footer">
    &copy; {year} NeuroCognica. {rights}
  </footer>
</main>
</body>
</html>
"#,
        body = body,
        chunk_count = context.chunk_count,
        classification = escape_html(&metadata.classification),
        date = escape_html(&metadata.date),
        engineer = escape_html(&metadata.engineer),
        format = PRINT_FORMAT_HTML,
        frame_id = escape_html(context.frame_id),
        generated_at = escape_html(&generated_at),
        kind = escape_html(kind),
        logo_asset = PRINT_LOGO_ASSET_NAME,
        logo_data_uri = context.logo_data_uri,
        logo_hash = context.logo_hash_blake3,
        metadata_hash = context.metadata_hash_blake3,
        normalized_text_bytes = context.normalized_text_bytes,
        page_profile = PRINT_PAGE_PROFILE,
        project = escape_html(&metadata.project),
        rights = escape_html(&metadata.rights),
        revision = escape_html(&metadata.revision),
        serialized_id = escape_html(&metadata.serialized_id),
        source_bytes = context.source_bytes,
        source_hash = context.source_hash_blake3,
        source_name = escape_html(context.source_name),
        source_path = escape_html(&source_path),
        style = PRINT_CSS,
        text_hash = context.text_hash_blake3,
        title = escape_html(&metadata.title),
        year = escape_html(&metadata.year),
    )
}

fn render_print_body(text: &str) -> String {
    let mut html = String::new();
    let mut paragraph = Vec::new();
    let mut list_open = false;
    let mut in_code = false;
    let mut code = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_code {
                flush_code(&mut code, &mut html);
                in_code = false;
            } else {
                flush_paragraph(&mut paragraph, &mut html);
                close_list(&mut list_open, &mut html);
                in_code = true;
            }
            continue;
        }

        if in_code {
            code.push_str(line);
            code.push('\n');
            continue;
        }

        if trimmed.is_empty() {
            flush_paragraph(&mut paragraph, &mut html);
            close_list(&mut list_open, &mut html);
            continue;
        }

        if let Some((level, heading)) = markdown_heading(trimmed) {
            flush_paragraph(&mut paragraph, &mut html);
            close_list(&mut list_open, &mut html);
            html.push_str(&format!("<h{level}>{}</h{level}>\n", escape_html(heading)));
            continue;
        }

        if let Some(item) = markdown_bullet(trimmed) {
            flush_paragraph(&mut paragraph, &mut html);
            if !list_open {
                html.push_str("<ul>\n");
                list_open = true;
            }
            html.push_str(&format!("  <li>{}</li>\n", escape_html(item)));
            continue;
        }

        close_list(&mut list_open, &mut html);
        paragraph.push(escape_html(trimmed));
    }

    if in_code {
        flush_code(&mut code, &mut html);
    }
    flush_paragraph(&mut paragraph, &mut html);
    close_list(&mut list_open, &mut html);

    if html.trim().is_empty() {
        "<p>No printable body content.</p>\n".to_owned()
    } else {
        html
    }
}

fn flush_paragraph(paragraph: &mut Vec<String>, html: &mut String) {
    if paragraph.is_empty() {
        return;
    }
    html.push_str("<p>");
    html.push_str(&paragraph.join(" "));
    html.push_str("</p>\n");
    paragraph.clear();
}

fn flush_code(code: &mut String, html: &mut String) {
    html.push_str("<pre><code>");
    html.push_str(&escape_html(code.trim_end()));
    html.push_str("</code></pre>\n");
    code.clear();
}

fn close_list(list_open: &mut bool, html: &mut String) {
    if *list_open {
        html.push_str("</ul>\n");
        *list_open = false;
    }
}

fn markdown_heading(line: &str) -> Option<(usize, &str)> {
    let level = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=3).contains(&level) || line.as_bytes().get(level) != Some(&b' ') {
        return None;
    }
    Some((level, line[level + 1..].trim()))
}

fn markdown_bullet(line: &str) -> Option<&str> {
    line.strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))
        .map(str::trim)
        .filter(|item| !item.is_empty())
}

fn document_kind_label(kind: DocumentKind) -> &'static str {
    match kind {
        DocumentKind::Markdown => "Markdown",
        DocumentKind::PlainText => "Text",
        DocumentKind::Json => "JSON",
        DocumentKind::JsonLines => "JSONL",
        DocumentKind::Csv => "CSV",
        DocumentKind::Tsv => "TSV",
        DocumentKind::Toml => "TOML",
        DocumentKind::Yaml => "YAML",
    }
}

fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for character in input.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn normalize_text(input: &str) -> String {
    let without_bom = input.strip_prefix('\u{feff}').unwrap_or(input);
    without_bom.replace("\r\n", "\n").replace('\r', "\n")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ChunkSpan {
    char_start: usize,
    char_end: usize,
    byte_start: usize,
    byte_end: usize,
}

fn chunk_spans(text: &str, config: FramerConfig) -> Vec<ChunkSpan> {
    let chars = text.chars().collect::<Vec<_>>();
    let total = chars.len();
    let mut spans = Vec::new();
    let mut start = 0;

    while start < total {
        let hard_end = (start + config.max_chunk_chars).min(total);
        let end = if hard_end == total {
            hard_end
        } else {
            preferred_boundary(&chars, start, hard_end).unwrap_or(hard_end)
        };
        let end = if end <= start { hard_end } else { end };

        spans.push(ChunkSpan {
            char_start: start,
            char_end: end,
            byte_start: byte_index_for_char(text, start),
            byte_end: byte_index_for_char(text, end),
        });

        if end >= total {
            break;
        }
        let next_start = end.saturating_sub(config.overlap_chars);
        start = if next_start > start { next_start } else { end };
    }

    spans
}

fn preferred_boundary(chars: &[char], start: usize, hard_end: usize) -> Option<usize> {
    let minimum = start + ((hard_end - start) * 3 / 5);

    if hard_end >= 2 {
        for idx in ((minimum + 2)..=hard_end).rev() {
            if chars.get(idx - 2) == Some(&'\n') && chars.get(idx - 1) == Some(&'\n') {
                return Some(idx);
            }
        }
    }

    for idx in (minimum + 1..=hard_end).rev() {
        if chars.get(idx - 1) == Some(&'\n') {
            return Some(idx);
        }
    }

    for idx in (minimum + 1..=hard_end).rev() {
        if chars.get(idx - 1).is_some_and(|c| c.is_whitespace()) {
            return Some(idx);
        }
    }

    None
}

fn byte_index_for_char(text: &str, char_index: usize) -> usize {
    if char_index == 0 {
        return 0;
    }
    text.char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len())
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn blake3_with_prefix(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn genesis_hash() -> String {
    blake3_with_prefix(b"neurocognica.aura.document_forever.genesis.v1")
}

fn mmr_genesis_hash() -> String {
    blake3_with_prefix(b"neurocognica.aura.document_mmr.genesis.v1")
}

fn frames_path(root: &Path) -> PathBuf {
    root.join("document_frames.jsonl")
}

fn chunks_path(root: &Path) -> PathBuf {
    root.join("document_chunks.jsonl")
}

fn prints_path(root: &Path) -> PathBuf {
    root.join("document_prints.jsonl")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_text_with_neurocognica_metadata_and_stable_chunks() {
        let source = write_temp_doc(
            "frame",
            "md",
            "# AURA Document\n\nThe document framer is the first gate before RAG storage.\n\nThis paragraph is intentionally long enough to force a second chunk when the test uses a small chunk window. The content stays deterministic so hashes and chunk boundaries remain stable.\n",
        );
        let metadata = test_metadata("NC-DOC-AURA-001");

        let framed = frame_text_document(
            &source,
            metadata,
            FramerConfig {
                max_chunk_chars: 256,
                overlap_chars: 32,
            },
        )
        .expect("document frames");

        assert_eq!(framed.frame.schema_version, FRAME_SCHEMA_VERSION);
        assert_eq!(framed.frame.metadata.organization, "NEUROCOGNICA");
        assert_eq!(framed.frame.source_kind, DocumentKind::Markdown);
        assert_eq!(framed.frame.chunk_count, framed.chunks.len());
        assert!(framed.frame.frame_id.starts_with("ncdf-"));
        assert!(!framed.frame.source_hash_blake3.is_empty());
        assert!(framed.chunks.iter().all(|chunk| {
            chunk.frame_id == framed.frame.frame_id
                && chunk.chunk_id.starts_with(&framed.frame.frame_id)
        }));
        assert_eq!(framed.print.schema_version, PRINT_SCHEMA_VERSION);
        assert_eq!(framed.print.frame_id, framed.frame.frame_id);
        assert!(framed.print.print_id.starts_with("ncdp-"));
        assert!(framed.print.html.contains("data:image/png;base64,"));
        assert!(framed.print.html.contains("NEUROCOGNICA"));
        assert!(framed.print.html.contains("AURA"));
        assert!(framed
            .print
            .html
            .contains("Archetypes - Utilizing - Reflective - Architecture"));
        assert!(framed.print.html.contains("PROJECT:"));
        assert!(framed.print.html.contains("TITLE:"));
        assert!(framed.print.html.contains("DWG NO:"));
        assert!(framed.print.html.contains("@page"));
        assert!(framed
            .print
            .html
            .contains("The document framer is the first gate"));
    }

    #[test]
    fn store_ingest_is_idempotent_for_same_framed_document() {
        let root = temp_root("store");
        let source = write_temp_doc(
            "store",
            "txt",
            "AURA stores framed text after metadata, hashing, and chunking.\n\nRepeat ingest must not duplicate the database rows.\n",
        );
        let store = DocumentStore::open(root).expect("store opens");
        let config = FramerConfig::default();

        let first = store
            .ingest_text_path(&source, test_metadata("NC-DOC-AURA-002"), config)
            .expect("first ingest stores");
        let second = store
            .ingest_text_path(&source, test_metadata("NC-DOC-AURA-002"), config)
            .expect("second ingest is idempotent");
        let summary = store.summary().expect("summary");

        let IngestOutcome::Stored {
            frame_id,
            chunks_written,
            forever_sequence,
            forever_record_hash,
            ..
        } = first
        else {
            panic!("first ingest should store");
        };
        assert_eq!(forever_sequence, 1);
        assert!(forever_record_hash.starts_with("blake3:"));
        assert_eq!(
            second,
            IngestOutcome::AlreadyExists {
                frame_id,
                chunks_present: chunks_written,
                print_present: true,
                forever_present: true
            }
        );
        assert_eq!(summary.frame_count, 1);
        assert_eq!(summary.chunk_count, chunks_written);
        assert_eq!(summary.print_count, 1);
        assert_eq!(summary.forever_event_count, 1);
        assert!(summary.forever_tip_hash.starts_with("blake3:"));
        assert_eq!(summary.mmr_leaf_count, 1);
        assert!(summary.mmr_root_hash.starts_with("blake3:"));
        assert!(summary.mmr_synced);
        assert_eq!(summary.store_engine, DOCUMENT_STORE_ENGINE);
        assert!(summary.db_path.ends_with(DOCUMENT_ROCKSDB_DIR_NAME));
    }

    #[test]
    fn store_can_ingest_an_already_framed_document() {
        let root = temp_root("store-framed");
        let source = write_temp_doc(
            "store-framed",
            "md",
            "AURA frames first, then stores only after the memory-write gate authorizes.\n",
        );
        let framed = frame_text_document(
            &source,
            test_metadata("NC-DOC-AURA-004"),
            FramerConfig::default(),
        )
        .expect("document frames");
        let store = DocumentStore::open(root).expect("store opens");

        let outcome = store
            .ingest_framed_document(&framed)
            .expect("framed document stores");
        let summary = store.summary().expect("summary");

        assert_eq!(
            outcome,
            IngestOutcome::Stored {
                frame_id: framed.frame.frame_id.clone(),
                print_id: framed.print.print_id.clone(),
                chunks_written: framed.chunks.len(),
                forever_sequence: 1,
                forever_record_hash: store
                    .forever_record_for_frame(&framed.frame.frame_id)
                    .expect("forever lookup")
                    .expect("forever record")
                    .record_hash_blake3
            }
        );
        assert_eq!(summary.frame_count, 1);
        assert_eq!(summary.chunk_count, framed.chunks.len());
        assert_eq!(summary.print_count, 1);
        assert_eq!(summary.forever_event_count, 1);
        assert_eq!(summary.mmr_leaf_count, 1);
        assert!(summary.mmr_synced);

        let stored_frame = store
            .frame_record(&framed.frame.frame_id)
            .expect("frame lookup")
            .expect("stored frame");
        let stored_print = store
            .print_record_for_frame(&framed.frame.frame_id)
            .expect("print lookup")
            .expect("stored print");
        let report = store
            .verify_forever_chain()
            .expect("forever chain verifies");
        let mmr_report = store.verify_mmr().expect("document MMR verifies");
        let forever = store
            .forever_record_for_frame(&framed.frame.frame_id)
            .expect("forever lookup")
            .expect("forever record");
        assert_eq!(stored_frame, framed.frame);
        assert_eq!(stored_print.print_id, framed.print.print_id);
        assert!(stored_print.html.contains("data:image/png;base64,"));
        assert!(stored_print.html.contains("PROJECT:"));
        assert!(report.verified);
        assert!(mmr_report.verified);
        assert_eq!(mmr_report.leaf_count, 1);
        assert_eq!(mmr_report.root_hash, forever.mmr_root_hash_blake3);
        assert_eq!(forever.mmr_schema_version, DOCUMENT_MMR_SCHEMA_VERSION);
        assert_eq!(forever.mmr_leaf_index, 0);
        assert_eq!(forever.mmr_leaf_count, 1);
        assert!(forever.mmr_leaf_hash_blake3.starts_with("blake3:"));
    }

    #[test]
    fn rocksdb_store_persists_records_after_reopen() {
        let root = temp_root("rocksdb-persist");
        let source = write_temp_doc(
            "rocksdb-persist",
            "md",
            "# Persistence\n\nAURA must remember the framed document after the process lets go of the database handle.\n",
        );
        let framed = frame_text_document(
            &source,
            test_metadata("NC-DOC-AURA-005"),
            FramerConfig::default(),
        )
        .expect("document frames");
        let frame_id = framed.frame.frame_id.clone();
        let print_id = framed.print.print_id.clone();
        let chunks = framed.chunks.len();
        let first_record_hash = {
            let store = DocumentStore::open(&root).expect("store opens");
            let outcome = store
                .ingest_framed_document(&framed)
                .expect("framed document stores");
            assert!(store.db_path().exists());
            let IngestOutcome::Stored {
                forever_record_hash,
                forever_sequence,
                ..
            } = outcome
            else {
                panic!("first ingest should store");
            };
            assert_eq!(forever_sequence, 1);
            forever_record_hash
        };

        let reopened = DocumentStore::open(&root).expect("store reopens");
        let summary = reopened.summary().expect("summary");
        let stored_frame = reopened
            .frame_record(&frame_id)
            .expect("frame lookup")
            .expect("stored frame");
        let stored_print = reopened
            .print_record_for_frame(&frame_id)
            .expect("print lookup")
            .expect("stored print");
        let forever = reopened
            .forever_record_for_frame(&frame_id)
            .expect("forever lookup")
            .expect("forever record");
        let report = reopened.verify_forever_chain().expect("chain verifies");
        let mmr_report = reopened.verify_mmr().expect("MMR verifies");

        assert_eq!(summary.frame_count, 1);
        assert_eq!(summary.chunk_count, chunks);
        assert_eq!(summary.print_count, 1);
        assert_eq!(summary.forever_event_count, 1);
        assert_eq!(summary.mmr_leaf_count, 1);
        assert!(summary.mmr_synced);
        assert_eq!(stored_frame.frame_id, frame_id);
        assert_eq!(stored_print.print_id, print_id);
        assert_eq!(forever.record_hash_blake3, first_record_hash);
        assert_eq!(report.tip_hash, first_record_hash);
        assert_eq!(mmr_report.leaf_count, 1);
        assert_eq!(mmr_report.root_hash, summary.mmr_root_hash);
    }

    #[test]
    fn document_mmr_replays_multiple_ingests_after_reopen() {
        let root = temp_root("mmr-replay");
        let first_source = write_temp_doc(
            "mmr-one",
            "md",
            "# First\n\nAURA commits the first framed document into the accumulator.\n",
        );
        let second_source = write_temp_doc(
            "mmr-two",
            "txt",
            "A second framed document must merge with the first MMR leaf into a peak.\n",
        );
        let first_frame_id;
        let second_frame_id;
        {
            let store = DocumentStore::open(&root).expect("store opens");
            let first = store
                .ingest_text_path(
                    &first_source,
                    test_metadata("NC-DOC-AURA-MMR-001"),
                    FramerConfig::default(),
                )
                .expect("first document stores");
            let second = store
                .ingest_text_path(
                    &second_source,
                    test_metadata("NC-DOC-AURA-MMR-002"),
                    FramerConfig::default(),
                )
                .expect("second document stores");
            let IngestOutcome::Stored {
                frame_id: stored_first,
                forever_sequence: first_sequence,
                ..
            } = first
            else {
                panic!("first ingest should store");
            };
            let IngestOutcome::Stored {
                frame_id: stored_second,
                forever_sequence: second_sequence,
                ..
            } = second
            else {
                panic!("second ingest should store");
            };
            assert_eq!(first_sequence, 1);
            assert_eq!(second_sequence, 2);
            first_frame_id = stored_first;
            second_frame_id = stored_second;
            let live_report = store.verify_mmr().expect("live MMR verifies");
            assert_eq!(live_report.leaf_count, 2);
            assert_eq!(live_report.peak_count, 1);
        }

        let reopened = DocumentStore::open(&root).expect("store reopens");
        let summary = reopened.summary().expect("summary");
        let first_record = reopened
            .forever_record_for_frame(&first_frame_id)
            .expect("first forever lookup")
            .expect("first forever record");
        let second_record = reopened
            .forever_record_for_frame(&second_frame_id)
            .expect("second forever lookup")
            .expect("second forever record");
        let chain_report = reopened.verify_forever_chain().expect("chain verifies");
        let mmr_report = reopened.verify_mmr().expect("MMR verifies");

        assert_eq!(summary.forever_event_count, 2);
        assert_eq!(summary.mmr_leaf_count, 2);
        assert!(summary.mmr_synced);
        assert_eq!(chain_report.event_count, 2);
        assert_eq!(mmr_report.leaf_count, 2);
        assert_eq!(mmr_report.peak_count, 1);
        assert_eq!(mmr_report.root_hash, summary.mmr_root_hash);
        assert_eq!(first_record.mmr_leaf_index, 0);
        assert_eq!(first_record.mmr_leaf_count, 1);
        assert_eq!(second_record.mmr_leaf_index, 1);
        assert_eq!(second_record.mmr_leaf_count, 2);
        assert_eq!(second_record.mmr_root_hash_blake3, summary.mmr_root_hash);
        assert_ne!(
            first_record.mmr_root_hash_blake3,
            second_record.mmr_root_hash_blake3
        );
    }

    #[test]
    fn unsupported_sources_are_rejected_before_storage() {
        let source = write_temp_doc("unsupported", "png", "not really an image");
        let err = frame_text_document(
            &source,
            test_metadata("NC-DOC-AURA-003"),
            FramerConfig::default(),
        )
        .expect_err("png is not a supported text ingest source");

        assert!(matches!(err, DocumentError::UnsupportedSource(_)));
    }

    fn test_metadata(serialized_id: &str) -> NeurocognicaFrameMetadata {
        let mut metadata = NeurocognicaFrameMetadata::new(
            "AURA RAG Foundation",
            "Document Framer Test",
            serialized_id,
            "MICHAEL HOLT",
            "2026-07-22",
            "A.0",
            "PROPRIETARY & CONFIDENTIAL",
        );
        metadata.year = "2026".to_owned();
        metadata
    }

    fn write_temp_doc(stem: &str, ext: &str, contents: &str) -> PathBuf {
        let root = temp_root(stem);
        fs::create_dir_all(&root).expect("temp root");
        let path = root.join(format!("{stem}.{ext}"));
        fs::write(&path, contents).expect("temp source");
        path
    }

    fn temp_root(stem: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "aura-documents-{stem}-{}-{nanos}",
            std::process::id()
        ))
    }
}
