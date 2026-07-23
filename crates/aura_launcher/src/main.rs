//! AURA launcher — the first real product surface.
//!
//! This window is intentionally narrow: it exposes the current Sentinel-first runtime state and
//! button-drives boot continuation through `aura_runtime`. Under the current deny-all policy,
//! the boot action refuses before side effects.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;

use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
};

use aura_documents::{
    default_document_dir, frame_text_document, DocumentError, DocumentKind, DocumentStore,
    FramedDocument, FramerConfig, IngestOutcome, NeurocognicaFrameMetadata,
};
use aura_runtime::{
    AuraAction, AuraError, BootSupervisor, DecisionLog, EffectRequest, SentinelMode,
};
use bevy::{
    app::AppExit,
    prelude::*,
    text::DEFAULT_FONT_DATA,
    window::{PrimaryWindow, WindowResolution},
};
use chrono::Utc;
use serde_json::json;

use model::{decision_log_path, default_data_dir, LauncherSnapshot};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.012, 0.014, 0.018)))
        .insert_resource(IntroClock::default())
        .insert_resource(LauncherRuntime::start())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_owned(),
                name: Some("aura.launcher".to_owned()),
                resolution: WindowResolution::new(1240, 760),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                maximize_primary_window,
                spawn_launcher_camera,
                install_launcher_font,
                spawn_ui,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                advance_intro_clock,
                animate_intro_texts,
                animate_alive_indicator,
                handle_buttons,
                refresh_snapshot_texts,
            )
                .chain(),
        )
        .run();
}

const WINDOW_TITLE: &str = "AURA - Archetypes - Utilizing - Reflective - Architecture";
const BRAND_LOGO_ASSET: &str = "brand/neurocognica_logo_large.png";
const AURA_ACRONYM: &str = "Archetypes - Utilizing - Reflective - Architecture";
const TITLE_RGB: (f32, f32, f32) = (0.96, 0.94, 0.88);
const SUBTITLE_RGB: (f32, f32, f32) = (0.56, 0.78, 0.86);
const VERSION_RGB: (f32, f32, f32) = (0.70, 0.72, 0.70);
const ALIVE_TEXT_RGB: (f32, f32, f32) = (0.68, 0.86, 0.88);
const NEUROCOGNICA_RGB: (f32, f32, f32) = (0.93, 0.68, 0.32);
const TITLE_START_ALPHA: f32 = 0.62;
const SUBTITLE_START_ALPHA: f32 = 0.34;
const ALIVE_TEXT_START_ALPHA: f32 = 0.68;
const DOCUMENT_EXTENSIONS: &[&str] = &[
    "md", "markdown", "txt", "text", "json", "jsonl", "csv", "tsv", "toml", "yaml", "yml",
];

#[derive(Resource, Default)]
struct IntroClock {
    seconds: f32,
}

#[derive(Resource, Clone)]
struct LauncherFont {
    handle: Handle<Font>,
}

#[derive(Resource)]
struct LauncherRuntime {
    boot: Option<BootSupervisor>,
    data_dir: std::path::PathBuf,
    ledger_path: std::path::PathBuf,
    document_intake: DocumentIntakeState,
    boot_attempts: u64,
    last_event: String,
}

impl LauncherRuntime {
    fn start() -> Self {
        let data_dir = default_data_dir();
        let ledger_path = decision_log_path(&data_dir);
        match DecisionLog::open(&ledger_path) {
            Ok(log) => {
                let boot = BootSupervisor::start(SentinelMode::Enforce, Arc::new(log));
                Self {
                    boot: Some(boot),
                    data_dir,
                    ledger_path,
                    document_intake: DocumentIntakeState::default(),
                    boot_attempts: 0,
                    last_event: "launcher started; work mode remains gated".to_owned(),
                }
            }
            Err(error) => Self {
                boot: None,
                data_dir,
                ledger_path,
                document_intake: DocumentIntakeState::default(),
                boot_attempts: 0,
                last_event: format!("runtime refused to start: {error}"),
            },
        }
    }

    fn snapshot(&self) -> LauncherSnapshot {
        match &self.boot {
            Some(boot) => LauncherSnapshot::from_runtime(
                &boot.status(),
                &self.data_dir,
                &self.ledger_path,
                boot.broker().effects_executed(),
                self.boot_attempts,
                &self.document_intake.selection_line(),
                &self.document_intake.action_line(),
                &self.last_event,
            ),
            None => LauncherSnapshot::fatal(&self.ledger_path, &self.last_event),
        }
    }

    fn refresh(&mut self) {
        self.last_event = "status refreshed from local runtime".to_owned();
    }

    fn attempt_boot_continue(&mut self) {
        self.boot_attempts += 1;
        match self.boot.as_mut() {
            Some(boot) => match boot.try_continue_boot() {
                Ok(status) => {
                    self.last_event =
                        format!("boot continue authorized; phase is {:?}", status.phase);
                }
                Err(error) => {
                    self.last_event = format!("boot continue refused: {error}");
                }
            },
            None => {
                self.last_event = "boot continue refused: decision ledger unavailable".to_owned();
            }
        }
    }

    fn select_document_file(&mut self) {
        match rfd::FileDialog::new()
            .set_title("Select AURA document")
            .add_filter("AURA text documents", DOCUMENT_EXTENSIONS)
            .pick_file()
        {
            Some(path) => {
                self.document_intake.select_file(path);
                self.last_event = self.document_intake.action_line();
            }
            None => {
                self.document_intake.last_event =
                    "Document action: file selection canceled".to_owned();
                self.last_event = self.document_intake.action_line();
            }
        }
    }

    fn select_document_folder(&mut self) {
        match rfd::FileDialog::new()
            .set_title("Select AURA document folder")
            .pick_folder()
        {
            Some(path) => {
                self.document_intake.select_folder(path);
                self.last_event = self.document_intake.action_line();
            }
            None => {
                self.document_intake.last_event =
                    "Document action: folder selection canceled".to_owned();
                self.last_event = self.document_intake.action_line();
            }
        }
    }

    fn clear_document_selection(&mut self) {
        self.document_intake = DocumentIntakeState::default();
        self.last_event = "document intake selection cleared".to_owned();
    }

    fn attempt_document_frame(&mut self) {
        let Some(selection) = self.document_intake.selected.clone() else {
            self.set_document_event("Document action: frame refused; no source selected");
            return;
        };
        if !selection.is_supported_file() {
            self.set_document_event(selection.unsupported_action_message("frame"));
            return;
        }
        let Some(boot) = self.boot.as_ref() else {
            self.set_document_event("Document action: frame refused; runtime unavailable");
            return;
        };

        let path = selection.path.clone();
        let metadata = metadata_for_source(&path);
        let resource = document_source_resource(&path);
        let payload_hash = request_payload_hash("document-frame", &path);
        let actor_id = boot.actor_id();

        match boot.broker().execute(EffectRequest {
            action: AuraAction::DocumentFrame,
            resource: Some(resource),
            actor_id,
            declared_intent: "frame selected NeuroCognica document before RAG storage".into(),
            payload_hash,
            side_effect: Box::new(move || {
                let framed = frame_text_document(&path, metadata, FramerConfig::default())
                    .map_err(document_error)?;
                Ok(json!({
                    "frame_id": framed.frame.frame_id,
                    "chunks": framed.chunks.len(),
                    "source": framed.frame.source_name,
                }))
            }),
        }) {
            Ok(outcome) => {
                let frame_id = outcome
                    .result
                    .get("frame_id")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown-frame");
                let chunks = outcome
                    .result
                    .get("chunks")
                    .and_then(|value| value.as_u64())
                    .unwrap_or(0);
                self.set_document_event(format!(
                    "Document action: frame authorized | {frame_id} | chunks {chunks}"
                ));
            }
            Err(error) => {
                self.set_document_event(format!("Document action: frame refused: {error}"));
            }
        }
    }

    fn attempt_document_ingest(&mut self) {
        let Some(selection) = self.document_intake.selected.clone() else {
            self.set_document_event("Document action: ingest refused; no source selected");
            return;
        };
        if !selection.is_supported_file() {
            self.set_document_event(selection.unsupported_action_message("ingest"));
            return;
        }
        let Some(boot) = self.boot.as_ref() else {
            self.set_document_event("Document action: ingest refused; runtime unavailable");
            return;
        };

        let framed_slot: Arc<Mutex<Option<FramedDocument>>> = Arc::new(Mutex::new(None));
        let frame_slot = Arc::clone(&framed_slot);
        let frame_path = selection.path.clone();
        let metadata = metadata_for_source(&frame_path);
        let actor_id = boot.actor_id();

        let frame_result = boot.broker().execute(EffectRequest {
            action: AuraAction::DocumentFrame,
            resource: Some(document_source_resource(&frame_path)),
            actor_id,
            declared_intent: "frame selected document before authorized ingest".into(),
            payload_hash: request_payload_hash("document-ingest-frame", &frame_path),
            side_effect: Box::new(move || {
                let framed = frame_text_document(&frame_path, metadata, FramerConfig::default())
                    .map_err(document_error)?;
                let frame_id = framed.frame.frame_id.clone();
                let chunks = framed.chunks.len();
                *frame_slot.lock().map_err(|_| {
                    AuraError::InvalidRequest("document frame lock poisoned".into())
                })? = Some(framed);
                Ok(json!({
                    "frame_id": frame_id,
                    "chunks": chunks,
                }))
            }),
        });

        if let Err(error) = frame_result {
            self.set_document_event(format!(
                "Document action: ingest refused at source gate: {error}"
            ));
            return;
        }

        let framed = match framed_slot.lock() {
            Ok(mut guard) => guard.take(),
            Err(_) => {
                self.set_document_event("Document action: ingest refused; frame lock poisoned");
                return;
            }
        };
        let Some(framed) = framed else {
            self.set_document_event("Document action: ingest refused; no framed payload");
            return;
        };

        let frame_id = framed.frame.frame_id.clone();
        let frame_id_for_result = frame_id.clone();
        let source_name = framed.frame.source_name.clone();
        let chunks = framed.chunks.len();
        let data_dir = self.data_dir.clone();
        let store_resource = document_store_resource(&source_name);
        let ingest_payload_hash =
            request_payload_hash("document-ingest-store", Path::new(&frame_id));

        match boot.broker().execute(EffectRequest {
            action: AuraAction::DocumentIngest,
            resource: Some(store_resource),
            actor_id,
            declared_intent: "ingest framed document into the AURA document database".into(),
            payload_hash: ingest_payload_hash,
            side_effect: Box::new(move || {
                let store =
                    DocumentStore::open(default_document_dir(&data_dir)).map_err(document_error)?;
                let outcome = store
                    .ingest_framed_document(&framed)
                    .map_err(document_error)?;
                Ok(json!({
                    "outcome": ingest_outcome_label(&outcome),
                    "frame_id": frame_id_for_result,
                    "chunks": chunks,
                }))
            }),
        }) {
            Ok(outcome) => {
                let label = outcome
                    .result
                    .get("outcome")
                    .and_then(|value| value.as_str())
                    .unwrap_or("stored");
                self.set_document_event(format!(
                    "Document action: ingest authorized | {label} | {frame_id} | chunks {chunks}"
                ));
            }
            Err(error) => {
                self.set_document_event(format!(
                    "Document action: ingest refused at store gate after frame: {error}"
                ));
            }
        }
    }

    fn attempt_open_document_db(&mut self) {
        let Some(boot) = self.boot.as_ref() else {
            self.set_document_event("Document action: open DB refused; runtime unavailable");
            return;
        };
        let db_dir = default_document_dir(&self.data_dir);
        let display = db_dir.to_string_lossy().into_owned();
        match boot.broker().execute(EffectRequest {
            action: AuraAction::ProcessSpawn,
            resource: Some("aura://documents/db-folder".into()),
            actor_id: boot.actor_id(),
            declared_intent: "open AURA document database folder in Explorer".into(),
            payload_hash: request_payload_hash("document-open-db-folder", &db_dir),
            side_effect: Box::new(move || {
                Command::new("explorer")
                    .arg(&db_dir)
                    .spawn()
                    .map_err(|error| {
                        AuraError::InvalidRequest(format!("explorer failed: {error}"))
                    })?;
                Ok(json!({"opened": display}))
            }),
        }) {
            Ok(outcome) => {
                let opened = outcome
                    .result
                    .get("opened")
                    .and_then(|value| value.as_str())
                    .unwrap_or("document DB");
                self.set_document_event(format!("Document action: opened {opened}"));
            }
            Err(error) => {
                self.set_document_event(format!("Document action: open DB refused: {error}"));
            }
        }
    }

    fn set_document_event(&mut self, event: impl Into<String>) {
        self.document_intake.last_event = event.into();
        self.last_event = self.document_intake.action_line();
    }
}

#[derive(Debug, Clone)]
struct DocumentIntakeState {
    selected: Option<DocumentSourceSelection>,
    last_event: String,
}

impl Default for DocumentIntakeState {
    fn default() -> Self {
        Self {
            selected: None,
            last_event: "Document action: waiting for source selection".to_owned(),
        }
    }
}

impl DocumentIntakeState {
    fn select_file(&mut self, path: PathBuf) {
        let supported_kind = DocumentKind::from_path(&path);
        let selection = DocumentSourceSelection {
            path,
            source_type: DocumentSourceType::File,
            supported_kind,
        };
        self.last_event = if let Some(kind) = supported_kind {
            format!(
                "Document action: file queued for Sentinel-gated frame | {}",
                document_kind_label(kind)
            )
        } else {
            "Document action: file queued but unsupported by v0.1 text intake".to_owned()
        };
        self.selected = Some(selection);
    }

    fn select_folder(&mut self, path: PathBuf) {
        self.selected = Some(DocumentSourceSelection {
            path,
            source_type: DocumentSourceType::Folder,
            supported_kind: None,
        });
        self.last_event =
            "Document action: folder queued; recursive scan is protected and not live yet"
                .to_owned();
    }

    fn selection_line(&self) -> String {
        match &self.selected {
            Some(selection) => selection.selection_line(),
            None => "Document intake: no source selected".to_owned(),
        }
    }

    fn action_line(&self) -> String {
        self.last_event.clone()
    }
}

#[derive(Debug, Clone)]
struct DocumentSourceSelection {
    path: PathBuf,
    source_type: DocumentSourceType,
    supported_kind: Option<DocumentKind>,
}

impl DocumentSourceSelection {
    fn is_supported_file(&self) -> bool {
        matches!(self.source_type, DocumentSourceType::File) && self.supported_kind.is_some()
    }

    fn selection_line(&self) -> String {
        match self.source_type {
            DocumentSourceType::File => match self.supported_kind {
                Some(kind) => format!(
                    "Document intake: file queued | {} | {}",
                    document_kind_label(kind),
                    display_path(&self.path)
                ),
                None => format!(
                    "Document intake: unsupported file queued | {}",
                    display_path(&self.path)
                ),
            },
            DocumentSourceType::Folder => format!(
                "Document intake: folder queued | scan protected/not live | {}",
                display_path(&self.path)
            ),
        }
    }

    fn unsupported_action_message(&self, action: &str) -> String {
        match self.source_type {
            DocumentSourceType::Folder => format!(
                "Document action: {action} refused; folder scan/import is not live in this build"
            ),
            DocumentSourceType::File => format!(
                "Document action: {action} refused; unsupported source extension for {}",
                display_path(&self.path)
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DocumentSourceType {
    File,
    Folder,
}

#[derive(Component, Clone, Copy)]
enum SnapshotField {
    Version,
    Phase,
    Sentinel,
    Ledger,
    DocumentDb,
    DocumentGate,
    DocumentSelection,
    DocumentAction,
    Effects,
    Services,
    Message,
    LastEvent,
}

#[derive(Component, Clone, Copy)]
enum LauncherButton {
    Refresh,
    BootContinue,
    AddDocumentFile,
    AddDocumentFolder,
    FrameDocument,
    IngestDocument,
    OpenDocumentDb,
    ClearDocumentSelection,
    Quit,
}

#[derive(Component, Clone, Copy)]
struct IntroFade {
    rgb: (f32, f32, f32),
    start_alpha: f32,
    end_alpha: f32,
    delay: f32,
    duration: f32,
}

impl IntroFade {
    fn new(
        rgb: (f32, f32, f32),
        start_alpha: f32,
        end_alpha: f32,
        delay: f32,
        duration: f32,
    ) -> Self {
        Self {
            rgb,
            start_alpha,
            end_alpha,
            delay,
            duration,
        }
    }
}

#[derive(Component, Clone, Copy)]
struct AlivePulse {
    low_rgb: (f32, f32, f32),
    high_rgb: (f32, f32, f32),
    low_alpha: f32,
    high_alpha: f32,
    speed: f32,
}

#[derive(Component)]
struct LauncherUiCamera;

fn maximize_primary_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.single_mut() {
        window.set_maximized(true);
    }
}

fn spawn_launcher_camera(mut commands: Commands) {
    commands.spawn((Camera2d, LauncherUiCamera));
}

fn install_launcher_font(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let font = Font::try_from_bytes(DEFAULT_FONT_DATA.to_vec())
        .expect("Bevy built-in launcher font should always parse");
    commands.insert_resource(LauncherFont {
        handle: fonts.add(font),
    });
}

fn spawn_ui(
    mut commands: Commands,
    runtime: Res<LauncherRuntime>,
    font: Res<LauncherFont>,
    asset_server: Res<AssetServer>,
) {
    let snapshot = runtime.snapshot();
    let brand_logo = asset_server.load(BRAND_LOGO_ASSET);
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Val::Px(50.0), Val::Px(34.0)),
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.012, 0.014, 0.018)),
        ))
        .with_children(|root| {
            spawn_header(root, &snapshot, &font, brand_logo);
            spawn_status_surface(root, &snapshot, &font);
            spawn_buttons(root, &font);
        });
}

fn spawn_header(
    parent: &mut ChildSpawnerCommands,
    snapshot: &LauncherSnapshot,
    font: &LauncherFont,
    brand_logo: Handle<Image>,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(222.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(30.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|header| {
            header.spawn((
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(220.0),
                    flex_shrink: 0.0,
                    ..default()
                },
                ImageNode::new(brand_logo),
            ));
            header
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(8.0),
                        flex_grow: 1.0,
                        min_width: Val::Px(640.0),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|identity| {
                    identity.spawn((
                        Text::new("NeuroCognica"),
                        text_font(font, 24.0),
                        TextColor(color_with_alpha(NEUROCOGNICA_RGB, SUBTITLE_START_ALPHA)),
                        IntroFade::new(NEUROCOGNICA_RGB, SUBTITLE_START_ALPHA, 1.0, 0.08, 0.54),
                    ));
                    identity.spawn((
                        Text::new("AURA"),
                        text_font(font, 76.0),
                        TextColor(color_with_alpha(TITLE_RGB, TITLE_START_ALPHA)),
                        title_intro_fade(),
                    ));
                    identity.spawn((
                        Text::new(AURA_ACRONYM),
                        text_font(font, 24.0),
                        TextColor(color_with_alpha(SUBTITLE_RGB, SUBTITLE_START_ALPHA)),
                        IntroFade::new(SUBTITLE_RGB, SUBTITLE_START_ALPHA, 1.0, 0.18, 0.78),
                    ));
                    spawn_alive_indicator(identity, font);
                    identity.spawn((
                        Text::new(snapshot.version_line.clone()),
                        text_font(font, 15.0),
                        TextColor(color_with_alpha(VERSION_RGB, 0.0)),
                        IntroFade::new(VERSION_RGB, 0.0, 1.0, 0.98, 0.72),
                        SnapshotField::Version,
                    ));
                });
        });
}

fn spawn_alive_indicator(parent: &mut ChildSpawnerCommands, font: &LauncherFont) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                margin: UiRect::top(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|alive| {
            alive.spawn((
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.18, 0.58, 0.62, 0.32)),
                AlivePulse {
                    low_rgb: (0.14, 0.48, 0.52),
                    high_rgb: (0.94, 0.64, 0.34),
                    low_alpha: 0.58,
                    high_alpha: 0.92,
                    speed: 4.6,
                },
            ));
            alive.spawn((
                Text::new("LAUNCHER ALIVE"),
                text_font(font, 13.0),
                TextColor(color_with_alpha(ALIVE_TEXT_RGB, ALIVE_TEXT_START_ALPHA)),
                IntroFade::new(ALIVE_TEXT_RGB, ALIVE_TEXT_START_ALPHA, 1.0, 0.0, 0.52),
            ));
        });
}

fn spawn_status_surface(
    parent: &mut ChildSpawnerCommands,
    snapshot: &LauncherSnapshot,
    font: &LauncherFont,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|surface| {
            spawn_status_line(
                surface,
                SnapshotField::Phase,
                &snapshot.phase_line,
                true,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::Sentinel,
                &snapshot.sentinel_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::Ledger,
                &snapshot.ledger_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::DocumentDb,
                &snapshot.document_db_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::DocumentGate,
                &snapshot.document_gate_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::DocumentSelection,
                &snapshot.document_selection_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::DocumentAction,
                &snapshot.document_action_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::Effects,
                &snapshot.effects_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::Services,
                &snapshot.services_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::Message,
                &snapshot.message_line,
                false,
                font,
            );
            spawn_status_line(
                surface,
                SnapshotField::LastEvent,
                &snapshot.last_event_line,
                false,
                font,
            );
        });
}

fn spawn_status_line(
    parent: &mut ChildSpawnerCommands,
    field: SnapshotField,
    text: &str,
    primary: bool,
    font: &LauncherFont,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(if primary { 56.0 } else { 40.0 }),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(18.0), Val::Px(0.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(if primary {
                Color::srgba(0.09, 0.12, 0.14, 0.92)
            } else {
                Color::srgba(0.03, 0.04, 0.05, 0.90)
            }),
            BorderColor::all(if primary {
                Color::srgba(0.68, 0.54, 0.26, 0.88)
            } else {
                Color::srgba(0.21, 0.34, 0.38, 0.74)
            }),
        ))
        .with_children(|line| {
            line.spawn((
                Text::new(text.to_owned()),
                text_font(font, if primary { 22.0 } else { 16.0 }),
                TextColor(if primary {
                    Color::srgb(0.96, 0.90, 0.72)
                } else {
                    Color::srgb(0.80, 0.84, 0.82)
                }),
                field,
            ));
        });
}

fn spawn_buttons(parent: &mut ChildSpawnerCommands, font: &LauncherFont) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(14.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|bar| {
            spawn_button(bar, LauncherButton::AddDocumentFile, "ADD FILE", font);
            spawn_button(bar, LauncherButton::AddDocumentFolder, "ADD FOLDER", font);
            spawn_button(bar, LauncherButton::FrameDocument, "FRAME SELECTED", font);
            spawn_button(bar, LauncherButton::IngestDocument, "INGEST SELECTED", font);
            spawn_button(bar, LauncherButton::OpenDocumentDb, "OPEN DB FOLDER", font);
            spawn_button(bar, LauncherButton::ClearDocumentSelection, "CLEAR", font);
            spawn_button(
                bar,
                LauncherButton::BootContinue,
                "ATTEMPT BOOT CONTINUE",
                font,
            );
            spawn_button(bar, LauncherButton::Refresh, "REFRESH STATUS", font);
            spawn_button(bar, LauncherButton::Quit, "QUIT", font);
        });
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    action: LauncherButton,
    label: &str,
    font: &LauncherFont,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(match action {
                    LauncherButton::AddDocumentFile => 118.0,
                    LauncherButton::AddDocumentFolder => 136.0,
                    LauncherButton::FrameDocument => 176.0,
                    LauncherButton::IngestDocument => 184.0,
                    LauncherButton::OpenDocumentDb => 176.0,
                    LauncherButton::ClearDocumentSelection => 96.0,
                    LauncherButton::BootContinue => 250.0,
                    LauncherButton::Refresh => 176.0,
                    LauncherButton::Quit => 94.0,
                }),
                height: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(button_color(action, Interaction::None)),
            BorderColor::all(button_border(action, Interaction::None)),
            action,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label.to_owned()),
                text_font(font, 15.0),
                TextColor(Color::srgb(0.94, 0.94, 0.90)),
            ));
        });
}

fn handle_buttons(
    mut interactions: Query<
        (
            &Interaction,
            &LauncherButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        Changed<Interaction>,
    >,
    mut runtime: ResMut<LauncherRuntime>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (interaction, action, mut background, mut border) in &mut interactions {
        *background = BackgroundColor(button_color(*action, *interaction));
        *border = BorderColor::all(button_border(*action, *interaction));
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            LauncherButton::Refresh => runtime.refresh(),
            LauncherButton::BootContinue => runtime.attempt_boot_continue(),
            LauncherButton::AddDocumentFile => runtime.select_document_file(),
            LauncherButton::AddDocumentFolder => runtime.select_document_folder(),
            LauncherButton::FrameDocument => runtime.attempt_document_frame(),
            LauncherButton::IngestDocument => runtime.attempt_document_ingest(),
            LauncherButton::OpenDocumentDb => runtime.attempt_open_document_db(),
            LauncherButton::ClearDocumentSelection => runtime.clear_document_selection(),
            LauncherButton::Quit => {
                app_exit.write(AppExit::Success);
            }
        };
    }
}

fn refresh_snapshot_texts(
    runtime: Res<LauncherRuntime>,
    mut texts: Query<(&SnapshotField, &mut Text)>,
) {
    if !runtime.is_changed() {
        return;
    }
    let snapshot = runtime.snapshot();
    for (field, mut text) in &mut texts {
        text.0 = match field {
            SnapshotField::Version => snapshot.version_line.clone(),
            SnapshotField::Phase => snapshot.phase_line.clone(),
            SnapshotField::Sentinel => snapshot.sentinel_line.clone(),
            SnapshotField::Ledger => snapshot.ledger_line.clone(),
            SnapshotField::DocumentDb => snapshot.document_db_line.clone(),
            SnapshotField::DocumentGate => snapshot.document_gate_line.clone(),
            SnapshotField::DocumentSelection => snapshot.document_selection_line.clone(),
            SnapshotField::DocumentAction => snapshot.document_action_line.clone(),
            SnapshotField::Effects => snapshot.effects_line.clone(),
            SnapshotField::Services => snapshot.services_line.clone(),
            SnapshotField::Message => snapshot.message_line.clone(),
            SnapshotField::LastEvent => snapshot.last_event_line.clone(),
        };
    }
}

fn advance_intro_clock(time: Res<Time>, mut clock: ResMut<IntroClock>) {
    clock.seconds += time.delta_secs();
}

fn animate_intro_texts(clock: Res<IntroClock>, mut texts: Query<(&IntroFade, &mut TextColor)>) {
    for (fade, mut color) in &mut texts {
        color.0 = color_with_alpha(
            fade.rgb,
            fade_alpha(
                clock.seconds,
                fade.delay,
                fade.duration,
                fade.start_alpha,
                fade.end_alpha,
            ),
        );
    }
}

fn animate_alive_indicator(
    clock: Res<IntroClock>,
    mut indicators: Query<(&AlivePulse, &mut BackgroundColor)>,
) {
    for (pulse, mut color) in &mut indicators {
        let unit = pulse_unit(clock.seconds, pulse.speed);
        let rgb = lerp_rgb(pulse.low_rgb, pulse.high_rgb, unit);
        let alpha = lerp(pulse.low_alpha, pulse.high_alpha, unit);
        color.0 = Color::srgba(rgb.0, rgb.1, rgb.2, alpha);
    }
}

fn button_color(action: LauncherButton, interaction: Interaction) -> Color {
    let base = match action {
        LauncherButton::AddDocumentFile => (0.06, 0.13, 0.13),
        LauncherButton::AddDocumentFolder => (0.06, 0.13, 0.13),
        LauncherButton::FrameDocument => (0.13, 0.10, 0.05),
        LauncherButton::IngestDocument => (0.15, 0.08, 0.05),
        LauncherButton::OpenDocumentDb => (0.05, 0.10, 0.14),
        LauncherButton::ClearDocumentSelection => (0.08, 0.08, 0.07),
        LauncherButton::BootContinue => (0.17, 0.09, 0.06),
        LauncherButton::Refresh => (0.04, 0.11, 0.15),
        LauncherButton::Quit => (0.08, 0.04, 0.04),
    };
    match interaction {
        Interaction::Pressed => Color::srgba(base.0 + 0.16, base.1 + 0.10, base.2 + 0.08, 0.98),
        Interaction::Hovered => Color::srgba(base.0 + 0.08, base.1 + 0.06, base.2 + 0.05, 0.96),
        Interaction::None => Color::srgba(base.0, base.1, base.2, 0.92),
    }
}

fn button_border(action: LauncherButton, interaction: Interaction) -> Color {
    let color = match action {
        LauncherButton::AddDocumentFile => (0.36, 0.80, 0.78),
        LauncherButton::AddDocumentFolder => (0.36, 0.80, 0.78),
        LauncherButton::FrameDocument => (0.90, 0.68, 0.34),
        LauncherButton::IngestDocument => (0.92, 0.50, 0.32),
        LauncherButton::OpenDocumentDb => (0.42, 0.72, 0.92),
        LauncherButton::ClearDocumentSelection => (0.62, 0.62, 0.56),
        LauncherButton::BootContinue => (0.95, 0.64, 0.34),
        LauncherButton::Refresh => (0.35, 0.76, 0.86),
        LauncherButton::Quit => (0.82, 0.36, 0.32),
    };
    let alpha = match interaction {
        Interaction::Pressed => 1.0,
        Interaction::Hovered => 0.88,
        Interaction::None => 0.62,
    };
    Color::srgba(color.0, color.1, color.2, alpha)
}

fn metadata_for_source(path: &Path) -> NeurocognicaFrameMetadata {
    NeurocognicaFrameMetadata::new(
        "AURA RAG Foundation",
        source_title(path),
        serialized_id_for_source(path),
        "MICHAEL HOLT",
        Utc::now().format("%Y-%m-%d").to_string(),
        "A.0",
        "PROPRIETARY & CONFIDENTIAL",
    )
}

fn source_title(path: &Path) -> String {
    path.file_stem()
        .or_else(|| path.file_name())
        .map(|name| {
            name.to_string_lossy()
                .replace(['_', '-'], " ")
                .trim()
                .to_owned()
        })
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "AURA Document".to_owned())
}

fn serialized_id_for_source(path: &Path) -> String {
    let stem = path
        .file_stem()
        .or_else(|| path.file_name())
        .map(|name| name.to_string_lossy())
        .unwrap_or_else(|| "selected-source".into());
    let segment = safe_identifier_segment(&stem);
    format!("NC-AURA-DOC-{segment}")
}

fn safe_identifier_segment(raw: &str) -> String {
    let mut out = raw
        .chars()
        .filter_map(|ch| {
            if ch.is_ascii_alphanumeric() {
                Some(ch.to_ascii_uppercase())
            } else if ch.is_whitespace() || matches!(ch, '-' | '_') {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>();
    while out.contains("--") {
        out = out.replace("--", "-");
    }
    let out = out.trim_matches('-');
    if out.is_empty() {
        "SELECTED-SOURCE".to_owned()
    } else {
        out.chars().take(40).collect()
    }
}

fn document_source_resource(path: &Path) -> String {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy())
        .unwrap_or_else(|| "selected-source".into());
    format!("aura://documents/source/{}", safe_resource_segment(&name))
}

fn document_store_resource(source_name: &str) -> String {
    format!(
        "aura://documents/store/{}",
        safe_resource_segment(source_name)
    )
}

fn safe_resource_segment(raw: &str) -> String {
    let segment = raw
        .chars()
        .filter_map(|ch| {
            if ch.is_ascii_alphanumeric() {
                Some(ch.to_ascii_lowercase())
            } else if matches!(ch, '.' | '-' | '_') {
                Some(ch)
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>();
    if segment.is_empty() {
        "selected-source".to_owned()
    } else {
        segment.chars().take(96).collect()
    }
}

fn request_payload_hash(action: &str, path: &Path) -> String {
    let payload = format!("{action}|{}", path.to_string_lossy());
    format!("blake3:{}", blake3::hash(payload.as_bytes()).to_hex())
}

fn document_error(error: DocumentError) -> AuraError {
    AuraError::InvalidRequest(format!("document intake failed: {error}"))
}

fn ingest_outcome_label(outcome: &IngestOutcome) -> &'static str {
    match outcome {
        IngestOutcome::Stored { .. } => "stored",
        IngestOutcome::AlreadyExists { .. } => "already exists",
    }
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

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn color_with_alpha(rgb: (f32, f32, f32), alpha: f32) -> Color {
    Color::srgba(rgb.0, rgb.1, rgb.2, alpha)
}

fn text_font(font: &LauncherFont, font_size: f32) -> TextFont {
    TextFont {
        font: font.handle.clone(),
        font_size,
        ..default()
    }
}

fn title_intro_fade() -> IntroFade {
    IntroFade::new(TITLE_RGB, TITLE_START_ALPHA, 1.0, 0.0, 0.74)
}

fn fade_alpha(seconds: f32, delay: f32, duration: f32, start_alpha: f32, end_alpha: f32) -> f32 {
    if seconds <= delay {
        return start_alpha;
    }
    if duration <= f32::EPSILON {
        return end_alpha;
    }
    let progress = ((seconds - delay) / duration).clamp(0.0, 1.0);
    let eased = progress * progress * (3.0 - 2.0 * progress);
    lerp(start_alpha, end_alpha, eased)
}

fn pulse_unit(seconds: f32, speed: f32) -> f32 {
    ((seconds * speed).sin() + 1.0) * 0.5
}

fn lerp(start: f32, end: f32, unit: f32) -> f32 {
    start + (end - start) * unit
}

fn lerp_rgb(start: (f32, f32, f32), end: (f32, f32, f32), unit: f32) -> (f32, f32, f32) {
    (
        lerp(start.0, end.0, unit),
        lerp(start.1, end.1, unit),
        lerp(start.2, end.2, unit),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_waits_until_delay() {
        let alpha = fade_alpha(0.10, 0.40, 1.0, 0.04, 1.0);
        assert!((alpha - 0.04).abs() < f32::EPSILON);
    }

    #[test]
    fn title_fade_is_visible_on_first_frame() {
        let fade = title_intro_fade();
        assert!(fade.start_alpha >= 0.50);
        assert_eq!(fade.delay, 0.0);
    }

    #[test]
    fn acronym_stays_expanded_and_hyphenated() {
        assert_eq!(
            AURA_ACRONYM,
            "Archetypes - Utilizing - Reflective - Architecture"
        );
        assert!(WINDOW_TITLE.contains(AURA_ACRONYM));
    }

    #[test]
    fn brand_logo_path_is_asset_relative() {
        assert_eq!(BRAND_LOGO_ASSET, "brand/neurocognica_logo_large.png");
        assert!(!BRAND_LOGO_ASSET.contains(':'));
        assert!(!BRAND_LOGO_ASSET.starts_with('\\'));
    }

    #[test]
    fn document_serialized_id_uses_neurocognica_prefix() {
        let id = serialized_id_for_source(Path::new(r"C:\docs\My AURA Plan.md"));
        assert!(id.starts_with("NC-AURA-DOC-"));
        assert!(id
            .chars()
            .all(|ch| { ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '-' }));
    }

    #[test]
    fn document_resources_are_aura_uris() {
        let source = document_source_resource(Path::new(r"C:\docs\My AURA Plan.md"));
        let store = document_store_resource("My AURA Plan.md");
        assert_eq!(source, "aura://documents/source/my-aura-plan.md");
        assert_eq!(store, "aura://documents/store/my-aura-plan.md");
    }

    #[test]
    fn fade_reaches_target_after_duration() {
        let alpha = fade_alpha(2.0, 0.40, 1.0, 0.04, 1.0);
        assert!((alpha - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn pulse_unit_stays_visible_range() {
        for seconds in [0.0_f32, 0.35, 0.90, 1.75, 3.20] {
            let unit = pulse_unit(seconds, 4.6);
            assert!((0.0..=1.0).contains(&unit));
        }
    }

    #[test]
    fn embedded_launcher_font_parses() {
        assert!(Font::try_from_bytes(DEFAULT_FONT_DATA.to_vec()).is_ok());
    }
}
