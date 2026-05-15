//! Debug overlay plugin - FPS counter and performance metrics.
//!
//! Toggle visibility with backtick (`) key.

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin};
use bevy_egui::{egui, EguiContexts};

/// Plugin that adds a debug overlay with FPS and performance info.
pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(EntityCountDiagnosticsPlugin)
            .init_resource::<DebugOverlayState>()
            .add_systems(Update, debug_overlay_system);
    }
}

#[derive(Resource)]
pub struct DebugOverlayState {
    pub visible: bool,
    frame_count: u64,
    last_mem_update: f64,
    cached_mem_mb: f32,
}

impl Default for DebugOverlayState {
    fn default() -> Self {
        Self {
            visible: true, // Visible by default for debugging
            frame_count: 0,
            last_mem_update: 0.0,
            cached_mem_mb: 0.0,
        }
    }
}

/// Get JS heap memory usage in MB (WASM only).
#[cfg(target_arch = "wasm32")]
fn get_memory_mb() -> Option<f32> {
    let window = web_sys::window()?;
    let performance = window.performance()?;
    // Try to get memory info (Chrome only)
    let js_val = js_sys::Reflect::get(&performance, &"memory".into()).ok()?;
    if js_val.is_undefined() {
        return None;
    }
    let used = js_sys::Reflect::get(&js_val, &"usedJSHeapSize".into()).ok()?;
    let bytes = used.as_f64()?;
    Some((bytes / 1024.0 / 1024.0) as f32)
}

#[cfg(not(target_arch = "wasm32"))]
fn get_memory_mb() -> Option<f32> {
    None
}

fn debug_overlay_system(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    mut state: ResMut<DebugOverlayState>,
    time: Res<Time>,
) {
    let ctx = contexts.ctx_mut();

    // Toggle with backtick key
    ctx.input(|i| {
        if i.key_pressed(egui::Key::Backtick) {
            state.visible = !state.visible;
        }
    });
    if !state.visible {
        return;
    }
    state.frame_count += 1;
    // Update memory every ~1 second to avoid overhead
    let now = time.elapsed_secs_f64();
    if now - state.last_mem_update > 1.0 {
        state.cached_mem_mb = get_memory_mb().unwrap_or(0.0);
        state.last_mem_update = now;
    }
    let fps = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let frame_time = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let entity_count = diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|d| d.value())
        .unwrap_or(0.0) as u64;

    // FPS color: green > 55, yellow > 30, red otherwise
    let fps_color = if fps > 55.0 {
        egui::Color32::from_rgb(100, 255, 100)
    } else if fps > 30.0 {
        egui::Color32::from_rgb(255, 255, 100)
    } else {
        egui::Color32::from_rgb(255, 100, 100)
    };
    egui::Window::new("perf")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(8.0, 8.0))
        .resizable(false)
        .title_bar(false)
        .frame(egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 200))
            .inner_margin(egui::Margin::same(8.0))
            .rounding(egui::Rounding::same(4.0)))
        .show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 2.0);
            // FPS with color indicator
            ui.horizontal(|ui| {
                ui.monospace("FPS: ");
                ui.colored_label(fps_color, format!("{:.0}", fps));
            });
            ui.monospace(format!("Frame: {:.2}ms", frame_time));
            ui.monospace(format!("Entities: {}", entity_count));
            ui.monospace(format!("Time: {:.1}s", now));
            if state.cached_mem_mb > 0.0 {
                ui.monospace(format!("Heap: {:.0}MB", state.cached_mem_mb));
            }
            ui.add_space(4.0);
            ui.colored_label(
                egui::Color32::from_rgb(128, 128, 128),
                "Press ` to hide"
            );
        });
}
