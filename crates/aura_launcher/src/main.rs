//! AURA launcher — the first real product surface.
//!
//! This window is intentionally narrow: it exposes the current Sentinel-first runtime state and
//! button-drives boot continuation through `aura_runtime`. Under the current deny-all policy,
//! the boot action refuses before side effects.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;

use std::sync::Arc;

use aura_runtime::{BootSupervisor, DecisionLog, SentinelMode};
use bevy::{
    app::AppExit,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

use model::{decision_log_path, default_data_dir, LauncherSnapshot};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.012, 0.014, 0.018)))
        .insert_resource(IntroClock::default())
        .insert_resource(LauncherRuntime::start())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AURA — Sentinel Boot".to_owned(),
                name: Some("aura.launcher".to_owned()),
                resolution: WindowResolution::new(1240, 760),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (maximize_primary_window, spawn_ui).chain())
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

const TITLE_RGB: (f32, f32, f32) = (0.96, 0.94, 0.88);
const SUBTITLE_RGB: (f32, f32, f32) = (0.56, 0.78, 0.86);
const VERSION_RGB: (f32, f32, f32) = (0.70, 0.72, 0.70);
const ALIVE_TEXT_RGB: (f32, f32, f32) = (0.68, 0.86, 0.88);

#[derive(Resource, Default)]
struct IntroClock {
    seconds: f32,
}

#[derive(Resource)]
struct LauncherRuntime {
    boot: Option<BootSupervisor>,
    data_dir: std::path::PathBuf,
    ledger_path: std::path::PathBuf,
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
                    boot_attempts: 0,
                    last_event: "launcher started; work mode remains gated".to_owned(),
                }
            }
            Err(error) => Self {
                boot: None,
                data_dir,
                ledger_path,
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
}

#[derive(Component, Clone, Copy)]
enum SnapshotField {
    Version,
    Phase,
    Sentinel,
    Ledger,
    DocumentDb,
    DocumentGate,
    Effects,
    Services,
    Message,
    LastEvent,
}

#[derive(Component, Clone, Copy)]
enum LauncherButton {
    Refresh,
    BootContinue,
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

fn maximize_primary_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.single_mut() {
        window.set_maximized(true);
    }
}

fn spawn_ui(mut commands: Commands, runtime: Res<LauncherRuntime>) {
    let snapshot = runtime.snapshot();
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
            spawn_header(root, &snapshot);
            spawn_status_surface(root, &snapshot);
            spawn_buttons(root);
        });
}

fn spawn_header(parent: &mut ChildSpawnerCommands, snapshot: &LauncherSnapshot) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|header| {
            header.spawn((
                Text::new("AURA"),
                TextFont {
                    font_size: 68.0,
                    ..default()
                },
                TextColor(color_with_alpha(TITLE_RGB, 0.04)),
                IntroFade::new(TITLE_RGB, 0.04, 1.0, 0.06, 1.18),
            ));
            header.spawn((
                Text::new("Archetypes Utilizing Reflective Architecture"),
                TextFont {
                    font_size: 21.0,
                    ..default()
                },
                TextColor(color_with_alpha(SUBTITLE_RGB, 0.0)),
                IntroFade::new(SUBTITLE_RGB, 0.0, 1.0, 0.64, 0.86),
            ));
            spawn_alive_indicator(header);
            header.spawn((
                Text::new(snapshot.version_line.clone()),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(color_with_alpha(VERSION_RGB, 0.0)),
                IntroFade::new(VERSION_RGB, 0.0, 1.0, 0.98, 0.72),
                SnapshotField::Version,
            ));
        });
}

fn spawn_alive_indicator(parent: &mut ChildSpawnerCommands) {
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
                    low_alpha: 0.30,
                    high_alpha: 0.92,
                    speed: 4.6,
                },
            ));
            alive.spawn((
                Text::new("LAUNCHER ALIVE"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(color_with_alpha(ALIVE_TEXT_RGB, 0.0)),
                IntroFade::new(ALIVE_TEXT_RGB, 0.0, 1.0, 0.42, 0.68),
            ));
        });
}

fn spawn_status_surface(parent: &mut ChildSpawnerCommands, snapshot: &LauncherSnapshot) {
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
            spawn_status_line(surface, SnapshotField::Phase, &snapshot.phase_line, true);
            spawn_status_line(
                surface,
                SnapshotField::Sentinel,
                &snapshot.sentinel_line,
                false,
            );
            spawn_status_line(surface, SnapshotField::Ledger, &snapshot.ledger_line, false);
            spawn_status_line(
                surface,
                SnapshotField::DocumentDb,
                &snapshot.document_db_line,
                false,
            );
            spawn_status_line(
                surface,
                SnapshotField::DocumentGate,
                &snapshot.document_gate_line,
                false,
            );
            spawn_status_line(
                surface,
                SnapshotField::Effects,
                &snapshot.effects_line,
                false,
            );
            spawn_status_line(
                surface,
                SnapshotField::Services,
                &snapshot.services_line,
                false,
            );
            spawn_status_line(
                surface,
                SnapshotField::Message,
                &snapshot.message_line,
                false,
            );
            spawn_status_line(
                surface,
                SnapshotField::LastEvent,
                &snapshot.last_event_line,
                false,
            );
        });
}

fn spawn_status_line(
    parent: &mut ChildSpawnerCommands,
    field: SnapshotField,
    text: &str,
    primary: bool,
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
                TextFont {
                    font_size: if primary { 22.0 } else { 16.0 },
                    ..default()
                },
                TextColor(if primary {
                    Color::srgb(0.96, 0.90, 0.72)
                } else {
                    Color::srgb(0.80, 0.84, 0.82)
                }),
                field,
            ));
        });
}

fn spawn_buttons(parent: &mut ChildSpawnerCommands) {
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
            spawn_button(bar, LauncherButton::BootContinue, "ATTEMPT BOOT CONTINUE");
            spawn_button(bar, LauncherButton::Refresh, "REFRESH STATUS");
            spawn_button(bar, LauncherButton::Quit, "QUIT");
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, action: LauncherButton, label: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(match action {
                    LauncherButton::BootContinue => 286.0,
                    LauncherButton::Refresh => 194.0,
                    LauncherButton::Quit => 110.0,
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
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
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

fn color_with_alpha(rgb: (f32, f32, f32), alpha: f32) -> Color {
    Color::srgba(rgb.0, rgb.1, rgb.2, alpha)
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
}
