//! NeuroCognica document framing for AURA.
//!
//! The old `C:\AURA-Lab\Doc_Framer\nc-framer.py` script framed schematic PNGs with a
//! NeuroCognica title block. This crate preserves that required intake shape for text
//! documents: metadata first, deterministic hashes, stable chunks, then storage.

use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};

pub const FRAME_SCHEMA_VERSION: &str = "neurocognica.document_frame.v1";
pub const CHUNK_SCHEMA_VERSION: &str = "neurocognica.document_chunk.v1";
pub const DEFAULT_MAX_CHUNK_CHARS: usize = 1_600;
pub const DEFAULT_CHUNK_OVERLAP_CHARS: usize = 160;

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
    #[error(transparent)]
    Io(#[from] std::io::Error),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedDocument {
    pub frame: DocumentFrameRecord,
    pub chunks: Vec<DocumentChunkRecord>,
    pub normalized_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentStore {
    root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentStoreSummary {
    pub root: PathBuf,
    pub frames_path: PathBuf,
    pub chunks_path: PathBuf,
    pub frame_count: usize,
    pub chunk_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestOutcome {
    Stored {
        frame_id: String,
        chunks_written: usize,
    },
    AlreadyExists {
        frame_id: String,
        chunks_present: usize,
    },
}

impl DocumentStore {
    pub fn open(root: impl Into<PathBuf>) -> DocumentResult<Self> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn frames_path(&self) -> PathBuf {
        frames_path(&self.root)
    }

    pub fn chunks_path(&self) -> PathBuf {
        chunks_path(&self.root)
    }

    pub fn summary(&self) -> DocumentResult<DocumentStoreSummary> {
        Self::summary_at(&self.root)
    }

    pub fn summary_at(root: impl Into<PathBuf>) -> DocumentResult<DocumentStoreSummary> {
        let root = root.into();
        let frames_path = frames_path(&root);
        let chunks_path = chunks_path(&root);
        Ok(DocumentStoreSummary {
            root,
            frame_count: count_lines_if_exists(&frames_path)?,
            chunk_count: count_lines_if_exists(&chunks_path)?,
            frames_path,
            chunks_path,
        })
    }

    pub fn ingest_text_path(
        &self,
        source_path: &Path,
        metadata: NeurocognicaFrameMetadata,
        config: FramerConfig,
    ) -> DocumentResult<IngestOutcome> {
        let framed = frame_text_document(source_path, metadata, config)?;
        if self.contains_frame(&framed.frame.frame_id)? {
            let chunks_present = self.count_chunks_for_frame(&framed.frame.frame_id)?;
            return Ok(IngestOutcome::AlreadyExists {
                frame_id: framed.frame.frame_id,
                chunks_present,
            });
        }

        append_json_line(&self.frames_path(), &framed.frame)?;
        for chunk in &framed.chunks {
            append_json_line(&self.chunks_path(), chunk)?;
        }

        Ok(IngestOutcome::Stored {
            frame_id: framed.frame.frame_id,
            chunks_written: framed.chunks.len(),
        })
    }

    fn contains_frame(&self, frame_id: &str) -> DocumentResult<bool> {
        let path = self.frames_path();
        if !path.exists() {
            return Ok(false);
        }

        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let frame: DocumentFrameRecord = serde_json::from_str(&line)?;
            if frame.frame_id == frame_id {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn count_chunks_for_frame(&self, frame_id: &str) -> DocumentResult<usize> {
        let path = self.chunks_path();
        if !path.exists() {
            return Ok(0);
        }

        let mut count = 0;
        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let chunk: DocumentChunkRecord = serde_json::from_str(&line)?;
            if chunk.frame_id == frame_id {
                count += 1;
            }
        }
        Ok(count)
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
        created_at_utc: Utc::now(),
        metadata,
        provenance: FramerProvenance {
            framer: "aura_documents".to_owned(),
            framer_version: env!("CARGO_PKG_VERSION").to_owned(),
            inherited_from: r"C:\AURA-Lab\Doc_Framer\nc-framer.py".to_owned(),
            rule: "Every integrated AURA document must be framed before storage.".to_owned(),
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
        normalized_text,
    })
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

fn frames_path(root: &Path) -> PathBuf {
    root.join("document_frames.jsonl")
}

fn chunks_path(root: &Path) -> PathBuf {
    root.join("document_chunks.jsonl")
}

fn append_json_line<T: Serialize>(path: &Path, value: &T) -> DocumentResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    serde_json::to_writer(&mut file, value)?;
    file.write_all(b"\n")?;
    file.flush()?;
    Ok(())
}

fn count_lines_if_exists(path: &Path) -> DocumentResult<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let mut count = 0;
    for line in BufReader::new(File::open(path)?).lines() {
        if !line?.trim().is_empty() {
            count += 1;
        }
    }
    Ok(count)
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
        } = first
        else {
            panic!("first ingest should store");
        };
        assert_eq!(
            second,
            IngestOutcome::AlreadyExists {
                frame_id,
                chunks_present: chunks_written
            }
        );
        assert_eq!(summary.frame_count, 1);
        assert_eq!(summary.chunk_count, chunks_written);
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
