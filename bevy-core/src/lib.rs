//! Shared Bevy plugins for WASM visualization projects.
//!
//! Provides common functionality across flock, pipedream, and spot:
//! - Pixel canvas for software rasterization
//! - Theme integration with ui-theme crate
//!
//! Performance overlay is now provided by `ui_theme::PerfOverlay`.
//! The debug module is available for opt-in entity/memory diagnostics.

pub mod debug;
pub mod pixel_canvas;
pub mod theme;

use bevy::prelude::*;

/// Bundle of common plugins for all WASM projects.
pub struct BevyCorePlugins;

impl Plugin for BevyCorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            theme::ThemePlugin,
        ));
    }
}

/// Re-export bevy_egui for convenience
pub use bevy_egui;
