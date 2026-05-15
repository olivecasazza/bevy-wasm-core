//! Shared egui styling and UI framework for WASM projects.
//!
//! Provides consistent high-contrast dark theme styling, responsive windows,
//! performance monitoring, and a `ProjectUi` abstraction that eliminates
//! boilerplate from individual projects.
//!
//! # Quick Start
//! ```rust
//! // In your state struct:
//! pub ui: ui_theme::ProjectUi,
//!
//! // Initialize:
//! ui: ui_theme::ProjectUi::new("my project"),
//!
//! // In your UI system:
//! if keyboard.just_pressed(KeyCode::Tab) { state.ui.toggle(); }
//! state.ui.frame(ctx, time.delta_secs(), |ui| {
//!     // project-specific controls here
//! });
//! ```

use std::collections::BTreeMap;
use egui::{Color32, Context, FontFamily, FontId, Margin, Stroke, TextStyle};

// Auto-generated theme values from themeColors.json
#[allow(dead_code)]
mod theme_gen {
    include!(concat!(env!("OUT_DIR"), "/theme_gen.rs"));
}

/// Semantic theme colors (from Tailwind config)
pub mod theme {
    use super::*;

    pub fn primary() -> Color32 {
        let (r, g, b) = theme_gen::PRIMARY_500;
        Color32::from_rgb(r, g, b)
    }

    pub fn secondary() -> Color32 {
        let (r, g, b) = theme_gen::SECONDARY_400;
        Color32::from_rgb(r, g, b)
    }

    pub fn highlight() -> Color32 {
        let (r, g, b) = theme_gen::HIGHLIGHT_300;
        Color32::from_rgb(r, g, b)
    }

    pub fn compliment() -> Color32 {
        let (r, g, b) = theme_gen::COMPLIMENT_300;
        Color32::from_rgb(r, g, b)
    }
}

/// UI color palette - high contrast dark theme
pub mod colors {
    use super::{Color32, Stroke};

    /// Semi-transparent black background (slightly see-through)
    pub fn bg() -> Color32 {
        Color32::from_rgba_unmultiplied(0, 0, 0, 210)
    }

    /// Standard text color (gray)
    pub fn text() -> Color32 {
        Color32::from_gray(160)
    }

    /// Hovered text color (lighter gray)
    pub fn text_hovered() -> Color32 {
        Color32::from_gray(220)
    }

    /// Active/pressed text color (near white)
    pub fn text_active() -> Color32 {
        Color32::from_gray(240)
    }

    /// Border color
    pub fn border_color() -> Color32 {
        Color32::from_gray(80)
    }

    /// Border stroke
    pub fn border() -> Stroke {
        Stroke::new(1.0, border_color())
    }
}

/// Standard corner rounding - square corners for industrial look
pub fn rounding() -> egui::Rounding {
    egui::Rounding::ZERO
}

/// Apply the shared high-contrast dark theme to the egui context.
pub fn apply_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = BTreeMap::from([
        (TextStyle::Heading, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Body, FontId::new(12.0, FontFamily::Monospace)),
        (TextStyle::Monospace, FontId::new(12.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(12.0, FontFamily::Monospace)),
        (TextStyle::Small, FontId::new(10.0, FontFamily::Monospace)),
    ]);

    style.visuals.window_fill = colors::bg();
    style.visuals.panel_fill = colors::bg();
    style.visuals.window_rounding = rounding();

    style.visuals.widgets.noninteractive.bg_fill = colors::bg();
    style.visuals.widgets.inactive.bg_fill = colors::bg();
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgba_unmultiplied(20, 20, 20, 210);
    style.visuals.widgets.active.bg_fill = Color32::from_rgba_unmultiplied(30, 30, 30, 210);

    style.visuals.widgets.noninteractive.rounding = rounding();
    style.visuals.widgets.inactive.rounding = rounding();
    style.visuals.widgets.hovered.rounding = rounding();
    style.visuals.widgets.active.rounding = rounding();

    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, colors::text());
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, colors::text());
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, colors::text_hovered());
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, colors::text_active());

    let no_stroke = Stroke::NONE;
    style.visuals.widgets.noninteractive.bg_stroke = no_stroke;
    style.visuals.widgets.inactive.bg_stroke = no_stroke;
    style.visuals.widgets.hovered.bg_stroke = no_stroke;
    style.visuals.widgets.active.bg_stroke = no_stroke;

    style.visuals.override_text_color = Some(colors::text());

    style.spacing.item_spacing = egui::vec2(6.0, 4.0);
    style.spacing.window_margin = Margin::same(8.0);
    style.spacing.button_padding = egui::vec2(6.0, 2.0);

    ctx.set_style(style);
}

/// Create a custom frame with horizontal-only borders (top and bottom)
fn horizontal_border_frame() -> egui::Frame {
    egui::Frame {
        fill: colors::bg(),
        rounding: rounding(),
        inner_margin: Margin::symmetric(8.0, 6.0),
        outer_margin: Margin::ZERO,
        stroke: Stroke::NONE,
        shadow: egui::Shadow::NONE,
    }
}

/// Paint horizontal borders (top and bottom only)
pub fn paint_horizontal_borders(ui: &mut egui::Ui, rect: egui::Rect) {
    let stroke = colors::border();
    let painter = ui.painter();
    painter.hline(rect.x_range(), rect.top(), stroke);
    painter.hline(rect.x_range(), rect.bottom(), stroke);
}

/// Create a styled window with the shared theme.
pub fn styled_window(title: &str) -> egui::Window<'_> {
    egui::Window::new(title)
        .frame(horizontal_border_frame())
        .collapsible(true)
        .default_open(true)
        .resizable(true)
}

/// Create a styled window with responsive positioning.
pub fn styled_window_responsive<'a>(ctx: &Context, title: &'a str) -> egui::Window<'a> {
    let screen = ctx.screen_rect();
    let is_mobile = screen.width() <= 768.0;

    let window = styled_window(title)
        .default_width(300.0);

    if is_mobile {
        window.anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -16.0))
    } else {
        window.anchor(egui::Align2::RIGHT_TOP, egui::vec2(-16.0, 16.0))
    }
}

// ─── Performance Overlay ────────────────────────────────────────────────────

const PERF_HISTORY_LEN: usize = 600;

/// Lightweight performance tracker.
///
/// Tracks FPS history and renders inline stats + `egui_plot` charts.
pub struct PerfOverlay {
    history: Vec<f32>,
    head: usize,
    count: usize,
    smoothed_fps: f32,
}

impl Default for PerfOverlay {
    fn default() -> Self {
        Self {
            history: vec![60.0; PERF_HISTORY_LEN],
            head: 0,
            count: 0,
            smoothed_fps: 60.0,
        }
    }
}

impl PerfOverlay {
    /// Record a new frame. Call once per frame with the delta time in seconds.
    pub fn update(&mut self, dt_seconds: f32) {
        let fps = if dt_seconds > 0.0 { 1.0 / dt_seconds } else { 0.0 };
        self.smoothed_fps = self.smoothed_fps * 0.9 + fps * 0.1;
        self.history[self.head] = self.smoothed_fps;
        self.head = (self.head + 1) % PERF_HISTORY_LEN;
        if self.count < PERF_HISTORY_LEN {
            self.count += 1;
        }
    }

    /// Render performance stats and FPS chart inline in an existing `egui::Ui`.
    pub fn ui(&self, ui: &mut egui::Ui) {
        let samples = self.count.max(1);

        let mut sum = 0.0_f32;
        let mut min_fps = f32::MAX;
        let mut max_fps = 0.0_f32;
        for i in 0..samples {
            let v = self.history[i];
            sum += v;
            if v < min_fps { min_fps = v; }
            if v > max_fps { max_fps = v; }
        }
        let avg_fps = sum / samples as f32;

        ui.label(format!(
            "fps {:>5.1}  avg {:>5.1}  min {:>5.1}",
            self.smoothed_fps, avg_fps, min_fps
        ));
        ui.label(format!(
            "frame {:.2} ms",
            if self.smoothed_fps > 0.0 { 1000.0 / self.smoothed_fps } else { 0.0 }
        ));

        ui.add_space(4.0);

        let line_color = if self.smoothed_fps >= 55.0 {
            theme::primary()
        } else if self.smoothed_fps >= 30.0 {
            theme::highlight()
        } else {
            theme::secondary()
        };

        // Downsample to every 4th sample for rendering perf
        let step = 4usize;
        let plot_count = (samples + step - 1) / step;
        let mut fps_points: Vec<[f64; 2]> = Vec::with_capacity(plot_count);
        let mut ms_points: Vec<[f64; 2]> = Vec::with_capacity(plot_count);
        for i in (0..samples).step_by(step) {
            let idx = if self.count < PERF_HISTORY_LEN {
                i
            } else {
                (self.head + i) % PERF_HISTORY_LEN
            };
            let fps = self.history[idx] as f64;
            let ms = if fps > 0.0 { 1000.0 / fps } else { 0.0 };
            fps_points.push([i as f64, fps]);
            ms_points.push([i as f64, ms]);
        }

        let fps_line = egui_plot::Line::new(egui_plot::PlotPoints::new(fps_points))
            .color(line_color)
            .fill(0.0)
            .name("fps");

        let ref_avg = egui_plot::HLine::new(avg_fps as f64)
            .color(Color32::from_rgba_premultiplied(80, 255, 80, 80))
            .style(egui_plot::LineStyle::dashed_dense())
            .name(format!("avg {:.0}", avg_fps));
        let ref_min = egui_plot::HLine::new(min_fps as f64)
            .color(Color32::from_rgba_premultiplied(255, 80, 80, 80))
            .style(egui_plot::LineStyle::dashed_dense())
            .name(format!("min {:.0}", min_fps));

        egui_plot::Plot::new("perf_fps_chart")
            .height(80.0)
            .show_axes([false, true])
            .show_grid(true)
            .allow_drag(true)
            .allow_zoom(true)
            .allow_scroll(true)
            .allow_boxed_zoom(true)
            .auto_bounds(egui::Vec2b::new(true, true))
            .legend(egui_plot::Legend::default().position(egui_plot::Corner::RightTop))
            .include_y(0.0)
            .include_y((max_fps + 10.0) as f64)
            .y_axis_label("fps")
            .show(ui, |plot_ui| {
                plot_ui.hline(ref_avg);
                plot_ui.hline(ref_min);
                plot_ui.line(fps_line);
            });

        ui.add_space(2.0);

        // ── Frame time (ms) chart ──
        let ms_color = Color32::from_rgba_premultiplied(200, 160, 255, 200);
        let ms_line = egui_plot::Line::new(egui_plot::PlotPoints::new(ms_points))
            .color(ms_color)
            .fill(0.0)
            .name("ms");

        let avg_ms = if avg_fps > 0.0 { 1000.0 / avg_fps } else { 0.0 };
        let max_ms = if min_fps > 0.0 { 1000.0 / min_fps } else { 0.0 };
        let ref_avg_ms = egui_plot::HLine::new(avg_ms as f64)
            .color(Color32::from_rgba_premultiplied(80, 255, 80, 80))
            .style(egui_plot::LineStyle::dashed_dense())
            .name(format!("avg {:.1}ms", avg_ms));

        egui_plot::Plot::new("perf_ms_chart")
            .height(60.0)
            .show_axes([false, true])
            .show_grid(true)
            .allow_drag(true)
            .allow_zoom(true)
            .allow_scroll(true)
            .allow_boxed_zoom(true)
            .auto_bounds(egui::Vec2b::new(true, true))
            .legend(egui_plot::Legend::default().position(egui_plot::Corner::RightTop))
            .include_y(0.0)
            .include_y((max_ms + 5.0) as f64)
            .y_axis_label("ms")
            .show(ui, |plot_ui| {
                plot_ui.hline(ref_avg_ms);
                plot_ui.line(ms_line);
            });
    }
}

// ─── Project UI ─────────────────────────────────────────────────────────────

/// Shared UI state for all WASM projects.
///
/// Encapsulates common boilerplate: visibility toggle, theme application,
/// performance monitoring, and responsive window creation.
///
/// # Usage
/// ```rust
/// // In your state struct:
/// pub ui: ui_theme::ProjectUi,
///
/// // Initialize:
/// ui: ui_theme::ProjectUi::new("my project"),
///
/// // In your UI system:
/// if keyboard.just_pressed(KeyCode::Tab) { state.ui.toggle(); }
/// state.ui.frame(ctx, time.delta_secs(), |ui| {
///     ui.collapsing("settings", |ui| { /* controls */ });
/// });
/// ```
pub struct ProjectUi {
    title: String,
    pub visible: bool,
    pub perf: PerfOverlay,
}

impl ProjectUi {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            visible: true,
            perf: PerfOverlay::default(),
        }
    }
}

impl Default for ProjectUi {
    fn default() -> Self {
        Self::new("settings")
    }
}

impl ProjectUi {
    /// Toggle UI visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Call once per frame. Applies styling, updates perf, and renders the
    /// settings window with a performance section automatically appended.
    ///
    /// The closure receives the `Ui` for project-specific controls.
    /// The performance section is added after the closure content.
    pub fn frame(&mut self, ctx: &Context, dt_seconds: f32, content: impl FnOnce(&mut egui::Ui)) {
        self.perf.update(dt_seconds);

        if !self.visible {
            return;
        }

        apply_style(ctx);

        styled_window_responsive(ctx, &self.title)
            .show(ctx, |ui| {
                content(ui);

                ui.collapsing("performance", |ui| {
                    self.perf.ui(ui);
                });
            });
    }
}
